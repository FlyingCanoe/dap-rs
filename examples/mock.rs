use std::collections::VecDeque;
use std::net;
use std::thread;

use anyhow::anyhow;
use dap::msg::dap_type::variable::VariableBuilder;
use dap::msg::request::ConfigurationDoneRequest;
use dap::msg::request::ContinueRequest;
use dap::msg::request::ContinueResponse;
use dap::msg::request::LaunchRequest;
use dap::msg::request::ScopesRequest;
use dap::msg::request::VariablesRequest;
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
        let context = Context::new(session);
        match run_session(context) {
            Ok(_) => {}
            Err(err) => {
                println!("error running session: {}", err)
            }
        }
    }
}

fn run_session(mut context: Context) -> anyhow::Result<()> {
    loop {
        thread::yield_now();
        for event in context.event_queue.drain(..) {
            context.session.send_event(event)?;
        }

        if let Some(request) = context.session.try_recv_request()? {
            match request {
                Request::Initialize(request) => context.handle_init_request(request)?,
                Request::Launch(request) => context.handle_lunch_request(request)?,
                Request::SetBreakpoints(request) => {
                    context.handle_set_breakpoints_request(request)?
                }
                Request::ConfigurationDone(request) => {
                    context.handle_configuration_done_request(request)?
                }
                Request::Threads(request) => context.handle_threads_request(request)?,
                Request::StackTrace(request) => context.handle_stack_trace_request(request)?,
                Request::Scopes(request) => context.handle_scopes_request(request)?,
                Request::Variables(request) => context.handle_variables_request(request)?,
                Request::Continue(request) => context.handle_continue_request(request)?,
                Request::Disconnect(request) => {
                    request.respond(Ok(()), &mut context.session)?;
                    break;
                }
                _ => println!("{request:#?}"),
            }
        } else {
            if context.is_running {
                context.generated_event();
            }
        }
    }
    Ok(())
}

struct Context {
    event_queue: Vec<Event>,
    breakpoint_list: Vec<Breakpoint>,
    is_running: bool,
    curent_pos: Option<Breakpoint>,
    session: Session,
}

impl Context {
    fn handle_init_request(&mut self, request: InitializeRequest) -> anyhow::Result<()> {
        let mut cap = Capabilities::default();
        cap.supports_configuration_done_request = Some(true);

        request.respond(
            Ok(InitializeResponse {
                capabilities: Some(cap.clone()),
            }),
            &mut self.session,
        )?;

        self.session.send_event(Event::Initialized)
    }

    fn handle_lunch_request(&mut self, request: LaunchRequest) -> anyhow::Result<()> {
        self.is_running = true;
        request.respond(Ok(()), &mut self.session)
    }

    fn handle_configuration_done_request(
        &mut self,
        request: ConfigurationDoneRequest,
    ) -> anyhow::Result<()> {
        request.respond(Ok(()), &mut self.session)
    }

    fn handle_set_breakpoints_request(
        &mut self,
        request: SetBreakpointsRequest,
    ) -> anyhow::Result<()> {
        let mut rng = rand::thread_rng();
        self.breakpoint_list = request
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

        println!("b len after={}", self.breakpoint_list.len());
        request.respond(
            Ok(SetBreakpointsResponse {
                breakpoints: self.breakpoint_list.clone(),
            }),
            &mut self.session,
        )
    }

    fn handle_threads_request(&mut self, request: ThreadsRequest) -> anyhow::Result<()> {
        request.respond(
            Ok(ThreadsResponse {
                threads: vec![Thread {
                    id: 0,
                    name: "main".to_string(),
                }],
            }),
            &mut self.session,
        )
    }

    fn handle_stack_trace_request(&mut self, request: StackTraceRequest) -> anyhow::Result<()> {
        let mut rng = rand::thread_rng();
        let mut stack_frames: VecDeque<_> = self
            .breakpoint_list
            .iter()
            .map(|breakpoint| {
                StackFrameBuilder::new(
                    rng.gen(),
                    "mock".to_string(),
                    breakpoint.line.unwrap_or(0),
                    breakpoint.column.unwrap_or(0),
                )
                .source(breakpoint.source.clone().unwrap())
                .build()
            })
            .collect();

        if let Some(breakpoint) = &self.curent_pos {
            stack_frames.push_front(
                StackFrameBuilder::new(
                    rng.gen(),
                    "mock".to_string(),
                    breakpoint.line.unwrap_or(0),
                    breakpoint.column.unwrap_or(0),
                )
                .source(
                    breakpoint
                        .source
                        .clone()
                        .ok_or(anyhow!("breakpoint without source are not suported"))?,
                )
                .build(),
            );
        }

        request.respond(
            Ok(StackTraceResponse {
                stack_frames: stack_frames.into(),
                total_frames: None,
            }),
            &mut self.session,
        )
    }

    fn handle_scopes_request(&mut self, request: ScopesRequest) -> anyhow::Result<()> {
        request.respond(
            Ok(ScopesResponse {
                scopes: vec![ScopeBuilder::new("Locals".to_string(), 1, false).build()],
            }),
            &mut self.session,
        )
    }

    fn handle_variables_request(&mut self, request: VariablesRequest) -> anyhow::Result<()> {
        request.respond(
            Ok(VariablesResponse {
                variables: vec![
                    VariableBuilder::new("mock".to_string(), 0, "var 1".to_string()).build(),
                ],
            }),
            &mut self.session,
        )
    }

    fn handle_continue_request(&mut self, request: ContinueRequest) -> anyhow::Result<()> {
        self.is_running = true;
        request.respond(
            Ok(ContinueResponse {
                all_threads_continued: Some(true),
            }),
            &mut self.session,
        )
    }

    fn new(session: Session) -> Context {
        Context {
            event_queue: vec![],
            breakpoint_list: vec![],
            is_running: false,
            session,
            curent_pos: None,
        }
    }

    fn generated_event(&mut self) {
        println!("b len in event={}", self.breakpoint_list.len());

        let mut rng = rand::thread_rng();
        if self.event_queue.is_empty() {
            if self.is_running && !self.breakpoint_list.is_empty() {
                let index = rng.gen_range(0..self.breakpoint_list.len());
                let breakpoint = &self.breakpoint_list[index];

                self.event_queue.push(Event::Stopped(StoppedEvent {
                    hit_breakpoint_ids: Some(vec![breakpoint.id.unwrap()]),
                    description: None,
                    text: None,
                    preserve_focus_hint: None,
                    all_threads_stopped: Some(true),
                    reason: event::stopped::Reason::Breakpoint,
                    thread_id: Some(0),
                }));

                self.curent_pos = Some(breakpoint.clone());
                self.is_running = false;
            }
        }
    }
}
