use std::collections::VecDeque;
use std::net::{TcpListener, TcpStream};

use crate::connection::SocketConnection;
use crate::msg::dap_type::Capabilities;
use crate::msg::request::{
    AcknowledgementResponse, ErrorResponse, InitializeResponse, LaunchRequest, Request,
};
use crate::msg::request::{Response, ResponseType};
use crate::msg::{Msg, MsgType};

pub struct DapCodec<'a> {
    listener: TcpListener,
    launch_request_handler: &'a dyn LaunchRequestHandler,
}

struct Session<'a> {
    connection: SocketConnection,
    launch_request_handler: &'a dyn LaunchRequestHandler,
    next_seq: u64,
}

impl<'a> DapCodec<'a> {
    pub fn new(listener: TcpListener) -> DapCodec<'a> {
        DapCodec {
            listener,
            launch_request_handler: &UnsupportedRequestHandler {},
        }
    }

    pub fn start(mut self) -> anyhow::Result<()> {
        self.listener.set_nonblocking(false)?;
        loop {
            let (connection, _) = self.listener.accept()?;
            let current_session = Session::new(connection, self.launch_request_handler)?;
            current_session.start(&mut self)?;
        }
    }

    pub fn set_launch_request_handler(
        &mut self,
        handler: &'a dyn LaunchRequestHandler,
    ) -> &mut Self {
        self.launch_request_handler = handler;
        self
    }
}

impl<'a> Session<'a> {
    fn new(
        socket: TcpStream,
        launch_request_handler: &'a dyn LaunchRequestHandler,
    ) -> anyhow::Result<Session> {
        let output = Session {
            connection: SocketConnection::new(socket)?,
            next_seq: 1,
            launch_request_handler,
        };
        Ok(output)
    }

    fn next_seq(&mut self) -> u64 {
        let output = self.next_seq;
        self.next_seq += 1;
        output
    }

    fn handle_unknown_request(&mut self, outgoing_msg_queue: &mut VecDeque<Msg>, request: &Msg) {
        outgoing_msg_queue.push_back(Msg {
            seq: self.next_seq(),
            msg_type: MsgType::Response(Response {
                request_seq: request.seq,
                response_type: ResponseType::Error(
                    ErrorResponse::new(Some("unsupported operation".to_string()), None)
                        .with_command(request.msg_type.as_request().unwrap().command().to_string()),
                ),
            }),
        })
    }

    fn handle_init_request(&mut self, outgoing_msg_queue: &mut VecDeque<Msg>, request: &Msg) {
        outgoing_msg_queue.push_back(Msg {
            seq: self.next_seq(),
            msg_type: MsgType::Response(Response {
                request_seq: request.seq,
                response_type: ResponseType::Initialize(InitializeResponse {
                    capabilities: Some(Capabilities::default()),
                }),
            }),
        })
    }

    fn start(mut self, _adapter: &mut DapCodec) -> anyhow::Result<()> {
        let mut outgoing_msg_queue = VecDeque::new();

        loop {
            for msg in outgoing_msg_queue.drain(..) {
                self.connection.send_msg(msg)?;
            }

            let incoming_msg = self.connection.read_msg()?;
            match incoming_msg.msg_type.clone() {
                MsgType::Request(request) => match request {
                    Request::Initialize(_) => {
                        self.handle_init_request(&mut outgoing_msg_queue, &incoming_msg)
                    }
                    Request::Launch(request) => {
                        let command = request.command().to_string();
                        let response_type =
                            match self.launch_request_handler.handle_request(request) {
                                Ok(_) => ResponseType::Acknowledgement(
                                    AcknowledgementResponse::new(command),
                                ),
                                Err(error) => ResponseType::Error(error.with_command(command)),
                            };

                        outgoing_msg_queue.push_back(Msg {
                            seq: self.next_seq(),
                            msg_type: MsgType::Response(Response {
                                request_seq: incoming_msg.seq,
                                response_type,
                            }),
                        })
                    }
                    _ => self.handle_unknown_request(&mut outgoing_msg_queue, &incoming_msg),
                },
                _ => {}
            }
        }
    }
}

struct UnsupportedRequestHandler;

pub trait LaunchRequestHandler {
    fn handle_request(&self, request: LaunchRequest) -> Result<(), ErrorResponse>;
}

impl LaunchRequestHandler for UnsupportedRequestHandler {
    fn handle_request(&self, _: LaunchRequest) -> Result<(), ErrorResponse> {
        Err(ErrorResponse::new(
            Some("unsupported request".to_string()),
            None,
        ))
    }
}
