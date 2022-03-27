use std::net;

use dap::codec::{DapCodec, LaunchRequestHandler};

struct Adapter {}

impl LaunchRequestHandler for Adapter {
    fn handle_request(
        &self,
        _: dap::msg::request::LaunchRequest,
    ) -> Result<(), dap::msg::request::ErrorResponse> {
        Ok(())
    }
}

fn main() {
    let listner = net::TcpListener::bind("127.0.0.1:4710").unwrap();
    let adapter = Adapter {};
    let mut codec = DapCodec::new(listner);
    codec.set_launch_request_handler(&adapter);
    codec.start().unwrap();
}
