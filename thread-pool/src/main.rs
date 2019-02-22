mod worker;
mod message;
mod thread_pool;

use std::thread;
use std::fs::File;
use std::io::prelude::*;
use std::time::Duration;
use thread_pool::ThreadPool;
use std::net::{ TcpListener, TcpStream };

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request {}", String::from_utf8_lossy(&buffer[..]));    
}

fn text_response(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn html_response(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let mut html_file = File::open("./source/index.html").unwrap();
    let mut read_result = String::new();
    html_file.read_to_string(&mut read_result).unwrap();

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", read_result);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_request(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let (response, file_path) = if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
        ("HTTP/1.1 200 OK", "./source/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "./source/404.html")
    };

    let mut html_file = File::open(file_path).unwrap();
    let mut read_result = String::new();
    html_file.read_to_string(&mut read_result).unwrap();

    let response = format!("{}\r\n\r\n{}", response, read_result);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn slow_request(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (head_response, file_path) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "./source/index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "./source/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "./source/404.html")
    };

    let mut html_file = File::open(file_path).unwrap();
    let mut read_result = String::new();
    html_file.read_to_string(&mut read_result).unwrap();

    let response = format!("{}\r\n\r\n{}", head_response, read_result);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);
    let mut counter = 0;

    for stream in listener.incoming() {
        if counter > 2 { break }
        counter += 1;

        pool.execute(|| {
            let stream = stream.unwrap();
            // handle_connection(&stream);
            // text_response(stream);
            // html_response(stream);
            // handle_request(stream);
            slow_request(stream);
        });
    }
}
