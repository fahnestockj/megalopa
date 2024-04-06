mod threads;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

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
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    println!("request_line {request_line}");
    // string parsing in rust... uhh
    // currently only supporting GET
    // TODO: True routing!!
    let (status_line, file_name) = if request_line == "GET / HTTP/1.1" {
        // parse out path
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(file_name).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
