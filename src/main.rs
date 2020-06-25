use tokio::prelude::*;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

use i8583::Unpacker;

#[tokio::main]
async fn main() {
    let mut listener = TcpListener::bind("0.0.0.0:5003").await.unwrap();

    loop {
        let (socket, _) = match listener.accept().await {
            Ok(val) => val,
            Err(e) => {
                eprintln!("Accept Error: {}", e);
                continue;
            }
        };

        tokio::spawn( async move {
            // Process each socket concurrently.
            handle_connection(socket).await
        });
    }
}

async fn handle_connection(mut stream: TcpStream) {
    
    // Read request
    // let mut request = Vec::with_capacity(1024);
    let mut request = [0; 4096];

    let _n = match stream.read(&mut request).await {
        // socket closed
        Ok(n) if n == 0 => return,
        Ok(n) => n,
        Err(e) => {
            eprintln!("failed to read from socket; err = {:?}", e);
            return;
        }
    };

    unpack_and_log(&request);

    // Forward message and receive response
    let mut destination = match TcpStream::connect("197.253.19.75:5003").await {
        Ok(destination) => destination,
        Err(e) => {
            eprintln!("Could not connect to destination server: {}", e);
            return;
        }
    };

    if let Err(e) = destination.write(&request).await {
        eprintln!("Could not write to destination server: {}", e);
        return;
    }

    let mut response = [0; 4096];
    if let Err(e) = destination.read(&mut response).await {
        eprintln!("Could not read from destination server: {}", e);
        return;
    }

    unpack_and_log(&response);

    // Return response to client
    if let Err(e) = stream.write(&response).await {
        eprintln!("Could not write to client: {}", e);
        return;
    }

    if let Err(e) = stream.flush().await {
        eprintln!("Could not flush stream: {}", e);
        return;
    }
}

fn unpack_and_log(iso: &[u8]) {
    println!("bytes: {}", String::from_utf8_lossy(&iso));

    // Unpack and log
    let mut unpacker = Unpacker::new(&iso[2..]);
    let fields = match unpacker.unpack(&i8583::nibss::SPEC) {
        Ok(fields) => fields,
        Err(e) => {
            eprintln!("Unpack error: {}", e);
            return;
        }
    };
    
    for (i, field) in fields.iter().enumerate() {
        if let Some(val) = field {
            println!("[{:03}] {}", i, String::from_utf8_lossy(val));
        }
    }
}
