use std::net;

use dap::codec::DapCodec;
use dap::msg::dap_type::capabilities::Capabilities;
use dap::msg::event::initialized::InitializedEvent;
use dap::msg::event::Event;
use dap::msg::request::{InitializeResponse, Request, RequestExt};

fn main() {
    let listner = net::TcpListener::bind("127.0.0.1:4711").unwrap();
    let mut codec = DapCodec::new(listner);
    let mut session = codec.accept().unwrap();

    let mut cap = Capabilities::default();
    cap.supports_function_breakpoints = Some(true);

    loop {
        let request = session.recv_request().unwrap();
        println!("{request:#?}");
        match request {
            Request::Initialize(init_request) => init_request
                .respond(
                    Ok(InitializeResponse {
                        capabilities: Some(cap.clone()),
                    }),
                    &mut session,
                )
                .unwrap(),
            Request::Launch(launch_request) => {
                launch_request.respond(Ok(()), &mut session).unwrap();
                session
                    .send_event(Event::Initialized(InitializedEvent {}))
                    .unwrap();
            }
            _ => println!("{request:#?}"),
        }
    }
}
