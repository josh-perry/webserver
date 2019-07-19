use std::str;
use std::path::Path;
use std::fs;
use std::io::prelude::*;
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream, Shutdown};

const BUFFER_SIZE: usize = 1024;

struct Request {
    body: String,
    headers: HashMap<String, String>
}

fn get_request(stream: &mut TcpStream) -> Request {
    let mut buf = [0u8; BUFFER_SIZE];

    loop {
        match stream.read(&mut buf) {
            Ok(size) => {
                if size < BUFFER_SIZE {
                    break
                }
            }
            Err(e) => {
                println!("Error reading stream: {}", e);
            }
        }
    }

    let request = str::from_utf8(&buf).unwrap();
    let mut headers = HashMap::new();

    for line in request.lines() {
        if !line.contains(":") {
            continue;
        }

        let split_header: Vec<&str> = line.splitn(2, ":").collect();
        headers.insert(split_header[0].trim().to_string(), split_header[1].trim().to_string());
    }

    Request {
        body: request.to_string(),
        headers: headers
    }
}

fn handle_client(mut stream: TcpStream) {
    let request = get_request(&mut stream);

    let header = String::from("HTTP/1.1 200 OK");

    let path = Path::new("example/index.html");
    let body = fs::read_to_string(path).expect("Failed to read file");

    let response = format!("{}\n\n{}", header, body);

    stream.write(response.as_bytes()).unwrap();
    stream.shutdown(Shutdown::Both).unwrap();
}

fn main() {
    let ip = "0.0.0.0";
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
