mod file_watcher;
mod http_header_utils;
mod threads;
use crate::cms::build;
use crate::utils::get_project_dir;
use crate::web_server::file_watcher::setup_file_watcher;
use crate::web_server::http_header_utils::parse_headers;
use http_bytes::http::{Response, StatusCode};
use http_bytes::response_header_to_vec;
use httparse;
use std::io::{BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::{self, fs};

pub fn start_dev_server(port: u16) {
    build();
    let mut addr = "127.0.0.1:".to_owned();
    addr.push_str(&port.to_string());

    let cwd = get_project_dir();
    let content_dir = cwd.join("content");
    setup_file_watcher(&content_dir.as_path()).unwrap();

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
    let cwd = get_project_dir();

    let buf_reader = BufReader::new(&mut stream);
    let mut req_buf = Vec::new();
    buf_reader.take(1000).read_to_end(&mut req_buf).unwrap();
    let mut headers = [httparse::EMPTY_HEADER; 100];

    let mut req = httparse::Request::new(&mut headers);
    req.parse(&req_buf).unwrap(); // TODO: handle better

    let r_headers = parse_headers(req.headers);

    if let (Some(req_method), Some(req_version), Some(req_path), Ok(headers)) =
        (req.method, req.version, req.path, r_headers)
    {
        dbg!(headers);
        if req_method == "GET" && req_version == 1 {
            // TODO: strip query instead of this check, or you know make an actual path parser
            if let None = req_path.find('?') {
                let path = cwd.join("public").join(req_path.strip_prefix("/").unwrap());
                handle_req(stream, path);
                return;
            }
        }
    }
    // 400 bad req

    let response_400: Response<()> = Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(())
        .unwrap();
    let res_vec = response_header_to_vec(&response_400);
    stream.write_all(&res_vec).unwrap();
}

fn handle_req(mut stream: TcpStream, mut path: PathBuf) {
    // if the path is a dir try to serve index.html
    if path.is_dir() {
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
            let mut res_vec = response_header_to_vec(&res);
            res_vec.write(contents).unwrap();
            stream.write_all(&res_vec).unwrap();
            stream.flush().unwrap();
        }
    } else {
        // it must be an html file!
        path.set_extension("html");
        if let Ok(contents) = fs::read(&path) {
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
            stream.flush().unwrap();
        }
    }

    // 404
    let response_404: Response<()> = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(())
        .unwrap();
    let res_vec = response_header_to_vec(&response_404);
    stream.write_all(&res_vec).unwrap();
    stream.flush().unwrap();
}
