use crate::app;
use crate::utils;

use tokio::prelude::*;
use tokio::net::TcpStream;


pub async fn handler(request: &[u8]) -> Result<Vec<u8>, String>
{
    utils::dump(&request[2..]);
    utils::unpack(&request[2..]);

    let host = match TcpStream::connect(app::HOST_ADDR.as_str()).await {
        Ok(host) => host,
        Err(e) => return Err(format!("Could not set up plain connection: {:?}", e))
    };

    let cx = match native_tls::TlsConnector::builder()
                .danger_accept_invalid_certs(app::CONFIG.accept_invalid_certs)
                .danger_accept_invalid_hostnames(app::CONFIG.accept_invalid_hostnames)
                .use_sni(app::CONFIG.use_sni)
                .build() {
        Ok(cx) => tokio_native_tls::TlsConnector::from(cx),
        Err(e) => return Err(format!("Could not build connector to host: {:?}", e))
    };

    let mut host = match cx.connect("197.253.19.75", host).await {
        Ok(host) => host,
        Err(e) => return Err(format!("Could set up TLS connection: {:?}", e))
    };

    if let Err(e) = host.write_all(request).await {
        return Err(format!("Could not write to host server: {:?}", e))
    }

    let response = utils::read_iso8583(&mut host).await?;
    utils::dump(&response[2..]);
    utils::unpack(&response[2..]);
    Ok(response)
}
