mod http_header_utils;
mod open_websocket;
mod threads;

use crate::build::build;
use crate::utils::get_project_dir;

use http_bytes::http::{Response, StatusCode};
use http_bytes::response_header_to_vec;
use httparse;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::{self, fs};

pub fn start_dev_server(port: u16) {
    build();
    let mut addr = "127.0.0.1:".to_owned();
    addr.push_str(&port.to_string());

    let listener = TcpListener::bind(&addr).unwrap();
    let pool = threads::ThreadPool::new(5);
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

    let mut recieved = [0u8; 1000];
    let bytes_peeked = stream.peek(&mut recieved).unwrap();
    assert_ne!(bytes_peeked, 0);
    stream.set_nonblocking(true).unwrap();

    let mut headers = [httparse::EMPTY_HEADER; 20];

    let mut req = httparse::Request::new(&mut headers);
    req.parse(&recieved).expect("http parse of request failed");
    let r_headers = http_header_utils::parse_headers(req.headers);
    if let (Some(req_method), Some(req_version), Some(mut req_path), Ok(headers)) =
        (req.method, req.version, req.path, r_headers)
    {
        if let (Some(_), Some(upgrade_header)) = (headers.get("Connection"), headers.get("Upgrade"))
        {
            if upgrade_header.eq(&"websocket") {
                open_websocket::open_websocket(&stream);
                return;
            }
        }
        if req_method == "GET" && req_version == 1 {
            // strip out query if there is one...
            if let Some(query_idx) = req_path.find('?') {
                req_path = &req_path[0..query_idx];
            }
            req_path = req_path
                .strip_prefix("/")
                .expect("req_path isn't relative...");
            let path = cwd.join("public").join(req_path);
            handle_req(stream, path);
            return;
        }
    }

    let response_400: Response<()> = Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(())
        .unwrap();
    let res_vec = response_header_to_vec(&response_400);
    stream.write_all(&res_vec).unwrap();
}

fn handle_req(mut stream: TcpStream, mut path: PathBuf) {
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

    let response_404: Response<()> = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(())
        .unwrap();
    let res_vec = response_header_to_vec(&response_404);
    stream.write_all(&res_vec).unwrap();
    stream.flush().unwrap();
}
