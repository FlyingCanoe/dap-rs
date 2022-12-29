use std::net;
use std::thread;

use dap::msg::dap_type::variable::VariableBuilder;
use dap::msg::request::ContinueResponse;
use dap::msg::request::VariablesResponse;
use rand::Rng;

use dap::codec::{DapCodec, Session};
use dap::msg::dap_type::breakpoint::Breakpoint;
use dap::msg::dap_type::breakpoint::BreakpointBuilder;
use dap::msg::dap_type::capabilities::Capabilities;
use dap::msg::dap_type::scope::ScopeBuilder;
use dap::msg::dap_type::stack_frame::StackFrameBuilder;
use dap::msg::dap_type::thread::Thread;
use dap::msg::event;
use dap::msg::event::stopped::StoppedEvent;
use dap::msg::event::Event;
use dap::msg::request::ScopesResponse;
use dap::msg::request::StackTraceRequest;
use dap::msg::request::StackTraceResponse;
use dap::msg::request::{
    InitializeRequest, InitializeResponse, Request, RequestExt, SetBreakpointsRequest,
    SetBreakpointsResponse, ThreadsRequest, ThreadsResponse,
};

fn main() {
    let listener = net::TcpListener::bind("127.0.0.1:4711").unwrap();
    let mut codec = DapCodec::new(listener);
    loop {
        let session = codec.accept().unwrap();
        run_session(session)
    }
}

fn run_session(mut session: Session) {
    let mut debuggee = Debuggee::new();

    loop {
        thread::yield_now();
        for event in debuggee.event_queue.drain(..) {
            session.send_event(event).unwrap();
        }

        if let Some(request) = session.try_recv_request().unwrap() {
            match request {
                Request::Initialize(request) => {
                    handle_init_request(request, &mut session);
                }
                Request::Launch(launch_request) => {
                    launch_request.respond(Ok(()), &mut session).unwrap();
                    debuggee.is_running = true;
                }
                Request::SetBreakpoints(request) => {
                    handle_set_breakpoints_request(request, &mut session, &mut debuggee)
                }
                Request::ConfigurationDone(request) => {
                    request.respond(Ok(()), &mut session).unwrap();
                }
                Request::Threads(request) => handle_threads_request(request, &mut session),
                Request::Disconnect(request) => {
                    request.respond(Ok(()), &mut session).unwrap();
                    break;
                }
                Request::StackTrace(request) => {
                    handle_stack_trace_request(request, &mut session, &debuggee)
                }
                Request::Scopes(request) => request
                    .respond(
                        Ok(ScopesResponse {
                            scopes: vec![ScopeBuilder::new("Locals".to_string(), 1, false).build()],
                        }),
                        &mut session,
                    )
                    .unwrap(),
                Request::Variables(request) => request
                    .respond(
                        Ok(VariablesResponse {
                            variables: vec![VariableBuilder::new(
                                "mock".to_string(),
                                0,
                                "var 1".to_string(),
                            )
                            .build()],
                        }),
                        &mut session,
                    )
                    .unwrap(),
                Request::Continue(request) => {
                    debuggee.is_running = true;
                    request
                        .respond(
                            Ok(ContinueResponse {
                                all_threads_continued: Some(true),
                            }),
                            &mut session,
                        )
                        .unwrap()
                }
                _ => println!("{request:#?}"),
            }
        } else {
            if debuggee.is_running {
                debuggee.generated_event();
            }
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
        .unwrap();

    session.send_event(Event::Initialized).unwrap();
}

fn handle_set_breakpoints_request(
    request: SetBreakpointsRequest,
    session: &mut Session,
    debuggee: &mut Debuggee,
) {
    let mut rng = rand::thread_rng();
    debuggee.breakpoint_list = request
        .breakpoints
        .clone()
        .unwrap_or_default()
        .into_iter()
        .map(|breakpoint| {
            BreakpointBuilder::new(true)
                .id(rng.gen())
                .line(breakpoint.line)
                .source(request.source.clone())
                .build()
        })
        .collect();

    request
        .respond(
            Ok(SetBreakpointsResponse {
                breakpoints: debuggee.breakpoint_list.clone(),
            }),
            session,
        )
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

fn handle_stack_trace_request(
    request: StackTraceRequest,
    session: &mut Session,
    debuggee: &Debuggee,
) {
    let mut rng = rand::thread_rng();
    let stack_frames: Vec<_> = debuggee
        .breakpoint_list
        .iter()
        .map(|breakpoint| {
            StackFrameBuilder::new(
                rng.gen(),
                "mock".to_string(),
                breakpoint.line.unwrap_or(1),
                breakpoint.column.unwrap_or(1),
            )
            .source(breakpoint.source.clone().unwrap())
            .build()
        })
        .collect();

    request
        .respond(
            Ok(StackTraceResponse {
                stack_frames: stack_frames,
                total_frames: None,
            }),
            session,
        )
        .unwrap()
}

struct Debuggee {
    event_queue: Vec<Event>,
    breakpoint_list: Vec<Breakpoint>,
    is_running: bool,
}

impl Debuggee {
    fn new() -> Debuggee {
        Debuggee {
            event_queue: vec![],
            breakpoint_list: vec![],
            is_running: false,
        }
    }

    fn generated_event(&mut self) {
        let mut rng = rand::thread_rng();
        if self.event_queue.is_empty() {
            if self.is_running && !self.breakpoint_list.is_empty() {
                let breakpoint =
                    &self.breakpoint_list[rng.gen_range(0..self.breakpoint_list.len())];
                self.event_queue.push(Event::Stopped(StoppedEvent {
                    hit_breakpoint_ids: Some(vec![breakpoint.id.unwrap()]),
                    description: None,
                    text: None,
                    preserve_focus_hint: None,
                    all_threads_stopped: Some(true),
                    reason: event::stopped::Reason::Breakpoint,
                    thread_id: Some(0),
                }));
                self.is_running = false;
            }
        }
    }
}
