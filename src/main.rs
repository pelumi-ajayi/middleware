use std::fs::File;
use std::io::{Read};
use middleware::app;

use tokio::net::TcpListener;
use native_tls::Identity;

use log::{info, error};

#[tokio::main]
async fn main() {
    env_logger::init();

    let listener = TcpListener::bind(format!("{}:{}", app::CONFIG.in_ip, app::CONFIG.in_port)).await.unwrap();

    let der = {
        let mut identity = vec![];
        let mut file = File::open(&app::CONFIG.cert).unwrap();
        file.read_to_end(&mut identity).unwrap();
        identity
    };

    let cert = Identity::from_pkcs12(&der, &app::CONFIG.cert_pin).unwrap();
    let tls_acceptor = tokio_native_tls::TlsAcceptor::from(native_tls::TlsAcceptor::builder(cert).build().unwrap());

    start_server(listener, tls_acceptor).await;
}

async fn start_server(mut listener: TcpListener, tls_acceptor: tokio_native_tls::TlsAcceptor) {
    info!("Server started... ");
    loop {
        let (socket, _) = match listener.accept().await {
            Ok(val) => val,
            Err(e) => {
                error!("Accept Error: {:?}", e);
                continue;
            }
        };

        let tls_acceptor = tls_acceptor.clone();
        tokio::spawn( async move {
            middleware::handle_connection(tls_acceptor, socket).await
        });       
    }
}
