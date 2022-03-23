use std::net;

use dap::adapter::Adapter;

fn main() {
    let listner = net::TcpListener::bind("127.0.0.1:4710").unwrap();
    let adapter = Adapter::new(listner);
    adapter.start().unwrap();
}
