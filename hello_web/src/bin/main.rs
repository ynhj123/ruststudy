use std::{fs, thread};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

use hello_web::ThreadPool;

fn main() {
    println!("Hello, world!");
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
        // thread::spawn(|| {
        //     handle_connection(stream);
        // })
        // handle_connection(stream);
        // println!("Connection established!");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    // let response = "HTTP/1.1 200 OK\r\n\r\n";
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}",
                           status_line,
                           contents.len(),
                           contents);
    // let contents = fs::read_to_string("hello.html").unwrap();
    //
    // let response = format!(
    //     "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
    //     contents.len(),
    //     contents
    // );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
