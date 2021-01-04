# middleware
Proxy iso8583 server.

# Dependencies
[i8583](http://github.com/O-Pelumi/i8583),
[tokio](http://github.com/tokio-rs/tokio),
[native-tls](http://github.com/sfackler/rust-native-tls),
[tokio-native-tls](https://github.com/tokio-rs/tls),
[futures](http://github.com/rust-lang/futures-rs),
[log](http://github.com/rust-lang/log),
[env_logger](http://github.com/env-logger-rs/env_logger),
[serde](http://github.com/serde-rs/serde),
[serde_json](http://github.com/serde-rs/json),
[lazy_static](http://github.com/rust-lang-nursery/lazy-static.rs)

# Setup
cd to the scripts directory and run ```generate-certificate.sh``` to generate a self-signed certificate for the proxy server, then set ```out_ip``` and ```out_port``` in rsc/config.json

# Usage
```
cargo run --release
```
To enable all logs
```
cargo build --release
RUST_LOG=middleware ./target/release/middleware 
```
