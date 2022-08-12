use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

fn main() {
    let mut count = 0;
    let max_clients = 20;

    loop {
        std::thread::spawn(move || create_stream_client());
        count += 1;

        if count >= max_clients {
            break;
        }
    }

    loop {}
}

fn create_stream_client() {
    let stream = TcpStream::connect("127.0.0.1:8080");

    match stream {
        Ok(stream) => loop {
            let stream_c = stream.try_clone().unwrap();

            let reply = stream_write_and_receive(stream_c, b"test");

            println!("Reply: {}", from_utf8(&reply).unwrap());
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn stream_write_and_receive(mut stream: TcpStream, data: &[u8]) -> [u8; 50] {
    stream.write(data).unwrap();

    return stream_read(stream);
}

fn stream_read(mut stream: TcpStream) -> [u8; 50] {
    let mut reply = [0 as u8; 50];

    loop {
        match stream.read(&mut reply) {
            Ok(size) => {
                break;
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    return reply;
}
