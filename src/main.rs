use std::io::{Read, Write, BufReader, BufRead};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:2345").unwrap();
    let stream = listener.accept().unwrap().0;
    handle_request(stream);
}

fn handle_request(stream: TcpStream) {

    let mut reader = BufReader::new(stream);

    for line in reader.by_ref().lines() {
        if line.unwrap() == "" {
            break;
        }
    }

    send_response(reader.into_inner());
}

fn send_response(mut stream: TcpStream) {
    // Write the header and the html body
    let response = "HTTP/1.1 200 OK\n\n<html><body><h1>Hello, World!</h1></body></html>";
    stream.write_all(response.as_bytes()).unwrap();
}

