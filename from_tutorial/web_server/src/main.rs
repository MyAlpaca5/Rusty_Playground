#![allow(dead_code)]

// implemented thread pool to handle connection concurrently
// there are other solutions, such as fork/join model, single-thread async I/O model, multi-threaded async I/O model.

use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let thread_pool = ThreadPool::new(3);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread_pool.execute(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);

    // get request content
    let request_line = reader.lines().next().unwrap().unwrap();

    let (status_line, content) = match &request_line[..] {
        "GET / HTTP/1.1" => {
            // generate response content
            let status_line = "HTTP/1.1 200 OK";
            let content = fs::read_to_string("./resources/hello.html").unwrap();
            (status_line, content)
        }
        "GET /sleep HTTP/1.1" => {
            // generate response content
            let status_line = "HTTP/1.1 200 OK";
            let content = fs::read_to_string("./resources/hello.html").unwrap();
            thread::sleep(Duration::from_secs(5));
            (status_line, content)
        }
        _ => {
            // generate response content
            let status_line = "HTTP/1.1 404 NOT FOUND";
            let content = fs::read_to_string("./resources/404.html").unwrap();
            (status_line, content)
        }
    };

    let length = content.len();
    let resp = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

    // send response
    stream.write_all(resp.as_bytes()).unwrap();
}
