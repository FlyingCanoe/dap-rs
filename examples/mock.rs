use std::net;

use dap::codec::DapCodec;
use dap::msg::dap_type::breakpoint::Breakpoint;
use dap::msg::dap_type::capabilities::Capabilities;
use dap::msg::event::initialized::InitializedEvent;
use dap::msg::event::Event;
use dap::msg::request::{InitializeResponse, Request, RequestExt, SetBreakpointsResponse};

fn main() {
    let listener = net::TcpListener::bind("127.0.0.1:4710").unwrap();
    let mut codec = DapCodec::new(listener);
    let mut session = codec.accept().unwrap();

    let mut cap = Capabilities::default();
    cap.supports_configuration_done_request = Some(true);

    loop {
        let request = session.recv_request().unwrap();
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
            Request::SetBreakpoints(set_breakpoint_request) => {
                let breakpoints = set_breakpoint_request
                    .breakpoints
                    .clone()
                    .into_iter()
                    .flat_map(Vec::into_iter)
                    .map(|breakpoint| Breakpoint {
                        source: Some(set_breakpoint_request.source.clone()),
                        column: breakpoint.column,
                        id: None,
                        instruction_reference: None,
                        line: Some(breakpoint.line),
                        end_column: None,
                        offset: None,
                        verified: true,
                        message: None,
                        end_line: None,
                    })
                    .collect();

                set_breakpoint_request
                    .respond(Ok(SetBreakpointsResponse { breakpoints }), &mut session)
                    .unwrap();
            }
            _ => println!("{request:#?}"),
        }
    }
}
