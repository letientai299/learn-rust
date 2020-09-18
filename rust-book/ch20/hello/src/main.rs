use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::fs;

use hello::ThreadPool;
use std::sync::Arc;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(200);
    let contents = fs::read_to_string("hello.html").unwrap();
    let c = Arc::new(contents);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let shared = Arc::clone(&c);
        pool.execute(|| {
            handle_connection(stream, shared);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream, c: Arc<String>) {
    let contents = &*c;
    let resp = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write(resp.as_bytes()).unwrap();
    stream.flush().unwrap();
}
