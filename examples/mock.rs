use std::net;

use dap::codec::{DapCodec, Session};
use dap::msg::dap_type::breakpoint::Breakpoint;
use dap::msg::dap_type::capabilities::Capabilities;
use dap::msg::dap_type::thread::Thread;
use dap::msg::event::Event;
use dap::msg::request::{
    InitializeRequest, InitializeResponse, Request, RequestExt, SetBreakpointsRequest,
    SetBreakpointsResponse, ThreadsRequest, ThreadsResponse,
};

fn main() {
    let listener = net::TcpListener::bind("127.0.0.1:4710").unwrap();
    let mut codec = DapCodec::new(listener);
    loop {
        let session = codec.accept().unwrap();
        run_session(session)
    }
}

fn run_session(mut session: Session) {
    loop {
        let request = session.recv_request().unwrap();
        match request {
            Request::Initialize(request) => handle_init_request(request, &mut session),
            Request::Launch(launch_request) => {
                launch_request.respond(Ok(()), &mut session).unwrap();
                session.send_event(Event::Initialized).unwrap();
            }
            Request::SetBreakpoints(request) => {
                handle_set_breakpoints_request(request, &mut session)
            }

            Request::ConfigurationDone(request) => request.respond(Ok(()), &mut session).unwrap(),
            Request::Threads(request) => handle_threads_request(request, &mut session),
            Request::Disconnect(request) => {
                request.respond(Ok(()), &mut session).unwrap();
                break;
            }
            _ => println!("{request:#?}"),
        }
    }
}

fn handle_init_request(request: InitializeRequest, session: &mut Session) {
    let mut cap = Capabilities::default();
    cap.supports_configuration_done_request = Some(true);

    request
        .respond(
            Ok(InitializeResponse {
                capabilities: Some(cap.clone()),
            }),
            session,
        )
        .unwrap()
}

fn handle_set_breakpoints_request(request: SetBreakpointsRequest, session: &mut Session) {
    let breakpoints = request
        .breakpoints
        .clone()
        .into_iter()
        .flat_map(Vec::into_iter)
        .map(|breakpoint| Breakpoint {
            source: Some(request.source.clone()),
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

    request
        .respond(Ok(SetBreakpointsResponse { breakpoints }), session)
        .unwrap();
}

fn handle_threads_request(request: ThreadsRequest, session: &mut Session) {
    request
        .respond(
            Ok(ThreadsResponse {
                threads: vec![Thread {
                    id: 0,
                    name: "main".to_string(),
                }],
            }),
            session,
        )
        .unwrap()
}
