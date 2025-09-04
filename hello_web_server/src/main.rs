use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream}
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let req_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match req_line.as_str() {
        "GET / HTTP/1.1" => {
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        }
    };

    let html = fs::read_to_string(filename).unwrap();
    let len = html.len();
    let res = format!("{status_line}\r\nContent-Length: {len}\r\n\r\n{html}");

    stream.write_all(res.as_bytes()).unwrap();
}
