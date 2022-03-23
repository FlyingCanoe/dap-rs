use std::net;

use dap::codec::DapCodec;

fn main() {
    let listener = net::TcpListener::bind("127.0.0.1:4710").unwrap();
    let codec = DapCodec::new(listener);
    codec.start().unwrap();
}
