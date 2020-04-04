use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let mut buffer = [0; 512];
    let result = stream.read(&mut buffer);

    let http_code = |n, msg| format!("HTTP/1.1 {} {}\r\n\r\n", n, msg);

    let (status_line, filename) = if result.is_err() {
        (http_code(500, "Internal Server Error"), "500.html")
    } else if buffer.starts_with(get) {
        (http_code(200, "OK"), "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        (http_code(200, "OK"), "hello.html")
    } else {
        (http_code(404, "NOT FOUND"), "404.html")
    };

    let contents = fs::read_to_string(String::from("src/") + filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
