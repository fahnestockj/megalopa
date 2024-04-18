mod file_watcher;
mod http_header_utils;
mod threads;
use crate::cms::build;
use crate::utils::get_project_dir;
use crate::web_server::file_watcher::setup_file_watcher;
use crate::web_server::http_header_utils::parse_headers;
use core::time;
use std::time::Duration;
use http_bytes::http::{Response, StatusCode};
use http_bytes::response_header_to_vec;
use httparse;
use notify::{RecursiveMode, Watcher};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::task::ready;
use std::thread::{panicking, sleep};
use std::{self, fs};
use tungstenite::protocol::frame::coding::CloseCode;
use tungstenite::protocol::CloseFrame;
use tungstenite::WebSocket;
use tungstenite::{accept, handshake::HandshakeRole, Error, HandshakeError, Message, Result};

pub fn start_dev_server(port: u16) {
    build();
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
    let cwd = get_project_dir();

    // let mut buf_reader = BufReader::new(&mut stream);
    // let recieved: Vec<u8> = buf_reader.fill_buf().unwrap().to_vec();
    // buf_reader.consume(recieved.len());

    // let mut recieved: Vec<u8> = vec![];
    let mut recieved = [0u8; 1000];
    let bytes_peeked = stream.peek(&mut recieved).unwrap();
    assert_ne!(bytes_peeked, 0);

    let mut headers = [httparse::EMPTY_HEADER; 100];

    let mut req = httparse::Request::new(&mut headers);
    req.parse(&recieved).expect("http parse of request failed"); // TODO: handle better
    let r_headers = parse_headers(req.headers);
    if let (Some(req_method), Some(req_version), Some(req_path), Ok(headers)) =
        (req.method, req.version, req.path, r_headers)
    {
        // setup websockets!
        if let (Some(_), Some(upgrade_header)) = (headers.get("Connection"), headers.get("Upgrade"))
        {
            if upgrade_header.eq(&"websocket") {
                // stream.set_nonblocking(true).unwrap();
                println!("WEBSOCKEN");
                println!("Why");

                let cwd = get_project_dir();
                let content_dir = cwd.join("content");

                let mut socket = accept(stream).unwrap();

                let (tx, rx) = std::sync::mpsc::channel::<bool>();

                // setup_file_watcher(&content_dir.as_path()).unwrap();
                let mut watcher = notify::recommended_watcher(move |res| match res {
                    Ok(event) => {
                        // println!("change detected: {:?}", event);
                        build();
                        tx.send(true).unwrap();
                    }
                    Err(e) => println!("watch error: {:?}", e),
                })
                .unwrap();
                watcher
                    .watch(content_dir.as_path(), RecursiveMode::Recursive)
                    .unwrap();

                loop {
                    let recieved_data = rx.recv().unwrap();
                    if recieved_data {
                        socket.send(Message::text(String::from("Reload!"))).unwrap();
                        break;
                    }
                }
                println!("After");
                return;
            }
        }
        if req_method == "GET" && req_version == 1 {
            // TODO: strip query instead of this check, or you know make an actual url parser
            if let None = req_path.find('?') {
                let path = cwd.join("public").join(req_path.strip_prefix("/").unwrap());
                handle_req(stream, path);
                return;
            }
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

    let response_404: Response<()> = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(())
        .unwrap();
    let res_vec = response_header_to_vec(&response_404);
    stream.write_all(&res_vec).unwrap();
    stream.flush().unwrap();
}
