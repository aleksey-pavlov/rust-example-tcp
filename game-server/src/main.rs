use std::net::TcpStream;
use std::{io::Read, io::Write, net::TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || client_handler(stream));
            }
            Err(e) => {
                println!("Error {}", e);
            }
        }
    }
}

fn client_handler(mut stream: TcpStream) {

    println!("New client connection {}", stream.peer_addr().unwrap());

    loop {
        let mut buffer = [0 as u8; 1024];

        match stream.read(&mut buffer) {
            Ok(size) => {
                stream.write(&buffer[0..size]).unwrap();
                if size <= 0 {
                    break;
                }
            }
            Err(e) => {
                println!("{}", e);
                break;
            }
        }
    }
}
