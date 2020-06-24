use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

use i8583::Unpacker;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5001").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    
    // Read request
    // let mut request = Vec::with_capacity(1024);
    let mut request = [0; 4096];

    if let Err(e) = stream.read(&mut request) {
        panic!("encountered IO error: {}", e)
    };
    println!("bytes: {}", std::str::from_utf8(&request).unwrap());

    // Unpack and log
    let mut unpacker = Unpacker::new(&request[2..]);
    let fields = unpacker.unpack(&i8583::nibss::SPEC).unwrap();
    for (i, field) in fields.iter().enumerate() {
        if let Some(val) = field {
            println!("[{:03}] {}", i, std::str::from_utf8(val).unwrap());
        }
    }

    // Forward message and receive response
    let mut destination = TcpStream::connect("197.253.19.75:5003").unwrap();
    println!("Connected to the server!");
    destination.write(&request).unwrap();
    let mut response = [0; 4096];
    destination.read(&mut response).unwrap();

    // Return response to client
    stream.write(&response).unwrap();
    stream.flush().unwrap();
}
