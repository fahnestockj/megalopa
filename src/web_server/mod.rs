mod threads;
use httparse;
use std::fs;
use std::io::{BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;

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
    let mut buf_reader = BufReader::new(&mut stream);
    let mut req_buf: Vec<u8> = Vec::new();
    buf_reader.read_to_end(&mut req_buf).unwrap();
    let mut headers = [httparse::EMPTY_HEADER; 8000]; // 8k? is this a good max

    let mut req = httparse::Request::new(&mut headers);
    req.parse(&req_buf).unwrap();

    // Ugly block checking all the parsing went well, and that the req is GET and http version 1
    // TODO: support query params
    if let Some(req_method) = req.method {
        if let Some(req_http_version) = req.version {
            if req_method == "GET" && req_http_version == 1 {
                if let Some(req_path) = req.path {
                    let r_query_idx = req_path.find('?');
                    match r_query_idx {
                        None => {
                            let path = PathBuf::from("public").join(req_path);
                            write_html_file_to_stream(stream, path);
                        }
                        Some(_) => {
                            panic!("Currently query strings are not supported")
                        }
                    }
                }
            }
        }
    }

    // 404!
    return;

    // println!("request_line {request_line}");
    // // string parsing in rust... uhh
    // // currently only supporting GET
    // // TODO: True routing!!
    // let (status_line, file_name) = if request_line == "GET / HTTP/1.1" {
    //     // parse out path
    //     ("HTTP/1.1 200 OK", "hello.html")
    // } else {
    //     ("HTTP/1.1 404 NOT FOUND", "404.html")
    // };
    // let contents = fs::read_to_string(file_name).unwrap();
    // let length = contents.len();
    // let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    // stream.write_all(response.as_bytes()).unwrap();
}

fn write_html_file_to_stream(mut stream: TcpStream, path: PathBuf) {
    let status_line = "HTTP/1.1 200 OK";
    // so the path either ends in html or we're serving index.html
    match path.extension() {
        Some(extension) => {
            if extension == "html" {
                let contents = fs::read_to_string(path).unwrap();
                let length = contents.len();
                let response =
                    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
                stream.write_all(response.as_bytes()).unwrap();
            } else {
                panic!("We don't handle anything else rn...")
            }
        }
        None => {
            //serve index.html
            let contents = fs::read_to_string(path).inspect_err(|err| {
                // TODO: 404?
                panic!("Couldn't find index.html for err: {}", err)
            }).unwrap();
            let length = contents.len();
            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}
