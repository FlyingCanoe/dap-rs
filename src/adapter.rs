use std::collections::VecDeque;
use std::net::{TcpListener, TcpStream};

use crate::connection::SocketConnection;
use crate::msg::dap_type::Capabilities;
use crate::msg::request::{ErrorResponse, InitializeResponse, Request};
use crate::msg::request::{Response, ResponseType};
use crate::msg::{Msg, MsgType};

#[derive(Debug)]
pub struct Adapter {
    listener: TcpListener,
}

#[derive(Debug)]
struct Session {
    connection: SocketConnection,
    next_seq: u64,
}

impl Adapter {
    pub fn new(listener: TcpListener) -> Adapter {
        Adapter { listener }
    }

    pub fn start(mut self) -> anyhow::Result<()> {
        self.listener.set_nonblocking(false)?;
        loop {
            let (connection, _) = self.listener.accept()?;
            let current_session = Session::new(connection)?;
            current_session.start(&mut self)?;
        }
    }
}

impl Session {
    fn new(socket: TcpStream) -> anyhow::Result<Session> {
        let output = Session {
            connection: SocketConnection::new(socket)?,
            next_seq: 1,
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

    fn start(mut self, _adapter: &mut Adapter) -> anyhow::Result<()> {
        let mut outgoing_msg_queue = VecDeque::new();

        loop {
            let incoming_msg = self.connection.read_msg()?;
            match &incoming_msg.msg_type {
                MsgType::Request(request) => match request {
                    Request::Initialize(_) => {
                        self.handle_init_request(&mut outgoing_msg_queue, &incoming_msg)
                    }
                    _ => self.handle_unknown_request(&mut outgoing_msg_queue, &incoming_msg),
                },
                _ => {}
            }

            for msg in outgoing_msg_queue.drain(..) {
                self.connection.send_msg(msg)?;
            }
        }
    }
}
