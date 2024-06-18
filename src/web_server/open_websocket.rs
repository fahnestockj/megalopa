use std::{net::TcpStream, time::Duration};
use notify::{RecursiveMode, Watcher};
use crate::{build, utils::get_project_dir};

pub fn open_websocket(stream: &TcpStream) -> () {
    let cwd = get_project_dir();
    let content_dir = cwd.join("content");

    let mut socket = tungstenite::accept(stream).unwrap();
    enum Message {
        FileChanged,
    }
    let (tx, rx) = std::sync::mpsc::channel::<Message>();
    let mut watcher =
        notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| match res {
            Ok(event) => {
                if event.kind.is_modify() {
                    build::build();
                    tx.send(Message::FileChanged).unwrap();
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        })
        .unwrap();
    watcher
        .watch(content_dir.as_path(), RecursiveMode::Recursive)
        .unwrap();

    println!("Websocket opening...");
    loop {
        // println!("Are we looping");
        // we swap between socket.read and seeing if we've recieved a notify message (that a file changed)
        let file_notify_msg = rx.recv_timeout(Duration::from_millis(10));
        match file_notify_msg {
            Ok(_) => {
                println!("File change detected, rebuilding...");
                socket
                    .send(tungstenite::Message::text(String::from("Reload!")))
                    .unwrap();
                break;
            }
            Err(_) => {}
        }
        let websocket_msg = socket.read();
        match websocket_msg {
            Ok(_) => {}
            Err(e) => match e {
                tungstenite::Error::AlreadyClosed => {
                    println!("Websocket connection closed");
                    break;
                }
                tungstenite::Error::ConnectionClosed => {
                    println!("Websocket connection closed");
                    break;
                }
                // std::io::ErrorKind::WouldBlock => {

                // }
                _ => {}
            },
        }
    }

    println!("Websocket closing...");
    return;
}
