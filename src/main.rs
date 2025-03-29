use std::fs;
use std::io::{prelude::*, BufReader};
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
    let first_line = request.lines().next().unwrap_or("");
    let path = first_line.split_whitespace().nth(1).unwrap_or("/");

    let mut filename = String::from("public");
    let (status_line, content_type) = if path == "/" {
        filename.push_str("/index.html");
        ("HTTP/1.1 200 OK", "text/html")
    } else if path.ends_with(".js") {
        filename.push_str(path);
        ("HTTP/1.1 200 OK", "application/javascript")
    } else {
        filename.push_str("/404.html");
        ("HTTP/1.1 404 NOT FOUND", "text/html")
    };

    // 🔥 Aggiungiamo questo per controllare il percorso
    println!("📂 Tentativo di apertura file: {}", filename);

    let contents = fs::read_to_string(filename.as_str()).unwrap_or_else(|_| {
        println!("❌ File non trovato: {}", filename); // Stampa errore se non lo trova
        String::from("File non trovato")
    });

    let response = format!(
        "{}\r\nContent-Type: {}; charset=UTF-8\r\n\r\n{}",
        status_line, content_type, contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
