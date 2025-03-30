use std::fs;
use std::io::{prelude::*};
use std::net::{ TcpStream};

pub fn resolve_url(path:&str, mut filename:String, mut stream:TcpStream){
    let (status_line, content_type) = match path {
        "/" => {
            filename.push_str("/index.html");
            ("HTTP/1.1 200 OK", "text/html")
        }
        "/home" =>{
            filename.push_str("/home.html");
            ("HTTP/1.1 200 OK", "text/html")
        }
        p if p.ends_with(".js") => {
            filename.push_str(p);
            ("HTTP/1.1 200 OK", "application/javascript")
        }
        _ => {
            filename.push_str("/404.html");
            ("HTTP/1.1 404 NOT FOUND", "text/html")
        }
    };
    println!("Searching in :{}",path);
    let contents = fs::read_to_string(filename.as_str()).unwrap_or_else(|_| {
        println!(" File non trovato: {}", filename);
        
        String::from("File non trovato")
    });

    let response = format!(
        "{}\r\nContent-Type: {}; charset=UTF-8\r\n\r\n{}",
        status_line, content_type, contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}