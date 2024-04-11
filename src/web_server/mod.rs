mod threads;
use http_bytes::http::{Response, StatusCode};
use http_bytes::response_header_to_vec;
use httparse;
use std::fs;
use std::io::{BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;

use crate::utils::get_project_cwd;

pub fn start_dev_server(port: u16) {
    let mut addr = "127.0.0.1:".to_owned();
    addr.push_str(&port.to_string());

    let listener = TcpListener::bind(&addr).unwrap();
    let pool = threads::ThreadPool::new(4);
    println!(
        "\n\n\tHosting a local web server at: http://localhost:{} \n\n",
        &port.to_string(),
    );
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        })
    }
    println!("Shutting down")
}

fn handle_connection(mut stream: TcpStream) {
    println!("started");
    let cwd = get_project_cwd().clone();

    let mut buf_reader = BufReader::new(&mut stream);
    let mut req_buf: Vec<u8> = Vec::new();
    buf_reader.read_to_end(&mut req_buf).unwrap();
    let mut headers = [httparse::EMPTY_HEADER; 8000]; // 8k? is this a good max

    let mut req = httparse::Request::new(&mut headers);
    req.parse(&req_buf).unwrap(); // TODO: handle better
                                  // let's confirm we have all needed and return a 400 Bad Request otherwise

    if let (Some(req_method), Some(req_version), Some(req_path)) =
        (req.method, req.version, req.path)
    {
        if req_method == "GET" && req_version == 1 {
            if let None = req_path.find('?') {
                let path = cwd.join("public").join(req_path.strip_prefix("/").unwrap());
                handle_req(stream, path);
                return;
            }
        }
    }
    // 500 bad req
    let res = Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(())
        .unwrap();
    let res_vec = response_header_to_vec(&res); // how do we include the body
    stream.write_all(&res_vec).unwrap();

}

fn handle_req(mut stream: TcpStream, path: PathBuf) {
    // so the path either ends in html or we're serving index.html
    if let Some(extension) = path.extension() {
        if extension == "html" {
            if let Ok(contents) = fs::read(path.clone()) {
                let length = contents.len();
                let res = Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "text/html")
                    .header("content-length", length)
                    .body(contents)
                    .unwrap();

                let mut res_vec = response_header_to_vec(&res);

                res_vec.write(res.body()).unwrap();
                stream.write_all(&res_vec).unwrap();
            }
        }
    }

    //try to serve index.html
    let index_path = path.join("index.html");
    if let Ok(r_contents) = fs::read_to_string(index_path) {
        let contents = r_contents.as_bytes();
        let length = contents.len();

        let res = Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "text/html")
            .header("content-length", length)
            .body(contents)
            .unwrap();
        let mut res_vec = response_header_to_vec(&res); // how do we include the body
        res_vec.write(contents).unwrap();
        stream.write_all(&res_vec).unwrap();
    }
    // 404
    let res = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(())
        .unwrap();
    let res_vec = response_header_to_vec(&res); // how do we include the body
    stream.write_all(&res_vec).unwrap();
}
