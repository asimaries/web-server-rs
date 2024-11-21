use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use web_server::ThreadPool;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Failed to read");

    // let request = String::from_utf8_lossy(&buffer[..]);
    // println!("LOG: {}", request);
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "static/index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "static/index.html")
    } else {
        ("HTTP/1.1 200 OK\r\n\r\n", "static/404.html")
    };
    let content = fs::read_to_string(filename).unwrap_or_default();

    let response = format!("{status_line} {content}");
    stream
        .write(response.as_bytes())
        .expect("Failed to write response");
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");

    println!("Server listening on http://localhost:8080");
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        match stream {
            Ok(stream) => {
               pool.execute(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e);
            }
        }
    }
}
