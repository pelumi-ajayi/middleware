use std::fs;
use serde::Deserialize;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG: Config = Config::new().unwrap(); 
    pub static ref HOST_ADDR: String = format!("{}:{}", CONFIG.out_ip, CONFIG.out_port); 
}

const CONFIG_FILE: &str = "rsc/config.json";

#[derive(Deserialize)]
pub struct Config {
    pub in_ip: String,
    pub in_port: String,
    pub out_ip: String,
    pub out_port: String,
    pub timeout: u32,
    pub cert: String,
    pub cert_pin: String,
    pub accept_invalid_certs: bool,
    pub accept_invalid_hostnames: bool,
    pub use_sni: bool
}

impl Config {
    fn new() -> Result<Config, Box<dyn std::error::Error>> {
        let json_str = fs::read_to_string(CONFIG_FILE)?;
        let config: Config = serde_json::from_str(&json_str)?;
        Ok(config)        
    }
}
