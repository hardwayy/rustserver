mod resolver;

use std::io::{prelude::*};
use std::net::{TcpListener, TcpStream};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server in ascolto su http://127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);


    let first_line = request.lines().next().unwrap_or("").to_string();
    let path = first_line.split_whitespace().nth(1).unwrap_or("/").to_string();

    let filename = String::from("public");

    resolver::resolve_url(&*path, filename, stream);
}


