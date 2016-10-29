use std::net::{TcpListener, TcpStream};
use std::thread;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:2345").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    println!("Success!");
                });
            }
            Err(e) => { println!("Failed");
            }
        }
    }
    drop(listener);
}
