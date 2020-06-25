use crate::utils;

pub async fn handler(request: &[u8]) -> Result<Vec<u8>, String>
{
    utils::dump(&request[2..]);
    utils::unpack(&request[2..]);
    Ok(request.to_vec())
}
