use std::{
    fs,
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

    let status_line = "HTTP/1.1 200 OK";

    // fetch the html file
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len(); // have to send the length of the response message body

    // formatted response
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
