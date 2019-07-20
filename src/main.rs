use std::str;
use std::path::Path;
use std::fs;
use std::fmt;
use std::io::prelude::*;
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream, Shutdown};

const BUFFER_SIZE: usize = 1024;

#[derive(Debug)]
enum Verb {
    GET,
    //HEAD,
    //POST,
    //PUT,
    //DELETE,
    //CONNECT,
    //OPTIONS,
    //TRACE,
    //PATCH
}

#[derive(Debug)]
struct Request {
    verb: Verb,
    path: String,
    body: String,
    headers: HashMap<String, String>
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
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
    let request_lines = request.lines();

    let mut verb = Verb::GET;
    let mut path = "".to_string();

    for line in request_lines {
        if !line.contains(":") {
            if line.starts_with("GET") {
                let split_line: Vec<&str> = line.splitn(3, " ").collect();

                verb = Verb::GET;
                path = split_line[1].trim().to_string();
            }

            continue;
        }

        let split_header: Vec<&str> = line.splitn(2, ":").collect();
        headers.insert(split_header[0].trim().to_string(), split_header[1].trim().to_string());
    }

    Request {
        body: request.trim_matches(char::from(0)).to_string(),
        headers: headers,
        verb: verb,
        path: path
    }
}

fn handle_client(mut stream: TcpStream) {
    let request = get_request(&mut stream);
    println!("{}", request.to_string());

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
