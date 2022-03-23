use std::net::{TcpListener, TcpStream};

use crate::connection::SocketConnection;
use crate::msg::dap_type::Capabilities;
use crate::msg::request::InitializeResponse;
use crate::msg::request::Response;
use crate::msg::{request, Msg, MsgType};

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

    fn start(mut self, _adapter: &mut Adapter) -> anyhow::Result<()> {
        let msg = self.connection.read_msg()?;

        let cap = Capabilities::default();

        let response = Msg {
            seq: self.next_seq(),
            msg_type: MsgType::Response(Response {
                request_seq: msg.seq,
                response_type: request::ResponseType::Initialize(InitializeResponse {
                    capabilities: Some(cap),
                }),
            }),
        };

        self.connection.send_msg(response)?;

        loop {
            let msg = self.connection.read_msg()?;
            println!("{msg:?}")
        }
    }
}
