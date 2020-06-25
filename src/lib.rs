pub mod app;

mod default;
mod utils;

use log::error;

use tokio::prelude::*;
use tokio::net::TcpStream;

pub async fn handle_connection(tls_acceptor: tokio_native_tls::TlsAcceptor, stream: TcpStream)
{
    // Accept the TLS connection.
    let mut tls_stream = match tls_acceptor.accept(stream).await {
        Ok(tls_stream) => tls_stream,
        Err(e) => {
            error!("TLS Accept Error: {:?}", e);
            return;
        }
    };

    let request = match utils::read_iso8583(&mut tls_stream).await {
        Ok(request) => request,
        Err(e) => {
            error!("{:?}", e);
            return;
        }
    };

    // TODO: Logic to determine handler
    // Send message to handler and receive response
    let response = match default::handler(&request).await {
        Ok(result) => result,
        Err(e) => {
            error!("{:?}", e);
            return;
        }
    };

    // Return response to client
    if let Err(e) = tls_stream.write_all(&response).await {
        error!("Could not write to client: {:?}", e);
    }

}
