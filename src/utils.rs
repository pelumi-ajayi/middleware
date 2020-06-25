use i8583::Unpacker;
use tokio::prelude::*;
use log::{info, error};

pub fn unpack(iso: &[u8])
{
    // Unpack and log
    let mut unpacker = Unpacker::new(iso);
    let fields = match unpacker.unpack(&i8583::nibss::SPEC) {
        Ok(fields) => fields,
        Err(e) => {
            error!("Unpack error: {:?}", e);
            return;
        }
    };
    
    for (i, field) in fields.iter().enumerate() {
        if let Some(val) = field {
            info!("[{:03}] {}", i, String::from_utf8_lossy(val));
        }
    }
}

pub fn dump(iso: &[u8])
{
    info!("{}", String::from_utf8_lossy(iso));
}

pub async fn read_iso8583(stream: &mut (impl AsyncRead + Unpin)) -> Result<Vec<u8>, String>
{
    let mut tpdu: [u8; 2] = [0; 2];
    
    let n = match stream.read(&mut tpdu).await {
        Ok(n) if n == 2 => {
            u16::from_be_bytes(tpdu) as usize
        }
        Ok(n) => return Err(format!("incomplete tpdu bytes with length {} read", n)),
        Err(e) => return Err(format!("TPDU stream read; {:?}", e))
    };

    let len = 2 + n;
    let mut iso = Vec::with_capacity(len);
    match tpdu.chain(stream).take(len as u64).read_to_end(&mut iso).await {
        Ok(count) if len == count => {
            Ok(iso)
        }
        Ok(count) => return Err(format!("Expected {} bytes, found {} bytes", count, len)),
        Err(e) => return Err(format!("iso8583 stream read; {:?}", e))
    }

}
