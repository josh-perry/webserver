use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, Shutdown};

fn handle_client(mut stream: TcpStream) {
    let hello_world = String::from("HTTP/1.1 200 OK\n\nHello World!");

    stream.write(hello_world.as_bytes()).unwrap();
    stream.shutdown(Shutdown::Both).unwrap();
}

fn main() {
    let ip = "127.0.0.1";
    let port = 8080;

    let listener = TcpListener::bind(format!("{}:{}", ip, port)).unwrap();

    println!("Listening on {}:{}", ip, port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection from {}", stream.peer_addr().unwrap());
                handle_client(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
