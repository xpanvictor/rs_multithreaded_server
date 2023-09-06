use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let tcp_listener = TcpListener::bind("127.0.0.1:4567").unwrap();

    for stream in tcp_listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request_uri = fetch_request_uri(&http_request[0]);
    println!("Got request for: {request_uri}");

    if request_uri == "/" {
        let status_line = "HTTP/1.1 200 OK";

        // fetch the html file
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len(); // have to send the length of the response message body

        // formatted response
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        stream.write_all(status_line.as_bytes()).unwrap();
    }
}

fn fetch_request_uri<'a>(request_line: &'a str) -> &'a str {
    let split_request_line: Vec<&str> = request_line.split_whitespace().collect();

    split_request_line[1]
}
