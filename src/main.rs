use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get_index = b"GET / HTTP/1.1\r\n";
    let get_jsx = b"GET /index.jsx HTTP/1.1\r\n";

    let (status_line, filename, content_type) = if buffer.starts_with(get_index) {
        ("HTTP/1.1 200 OK", "index.html", "text/html")
    } else if buffer.starts_with(get_jsx) {
        ("HTTP/1.1 200 OK", "index.jsx", "text/babel")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "public/404.html", "text/html")
    };

    let contents = fs::read_to_string(filename).unwrap_or_else(|_| String::from("File non trovato"));
    let response = format!(
        "{}\r\nContent-Type: {}; charset=UTF-8\r\n\r\n{}",
        status_line, content_type, contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}