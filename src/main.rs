use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let tcp_listener = TcpListener::bind("127.0.0.1:4567").unwrap();

    for stream in tcp_listener.incoming() {
        let stream = stream.unwrap();

        handleConnection(stream);
    }
}

fn handleConnection(mut stream: TcpStream) {
    let bufReader = BufReader::new(&mut stream);

    let http_request: Vec<_> = bufReader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = "HTTP/1.1 400 Bad\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}
