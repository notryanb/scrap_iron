use std::io::{Read, Write, BufReader, BufRead};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::path::Path;
use std::env;
use std::fs::File;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:2345").unwrap();
    let stream = listener.accept().unwrap().0;
    handle_request(stream);
}

fn handle_request(stream: TcpStream) {
    let path = Path::new("/public/");
    println!("Path: {:?} ", path);
    println!("Current dir {:?}", std::env::current_dir().unwrap());

    let mut reader = BufReader::new(stream);

    for line in reader.by_ref().lines() {
        if line.unwrap() == "" {
            break;
        }
    }

    send_response(reader.into_inner());
}

fn send_response(mut stream: TcpStream) {
    let mut f = File::open("public/index.html").unwrap();
    let mut response = String::new();

    f.read_to_string(&mut response);

    stream.write(response.as_bytes()).unwrap();
}

