use std::net::{TcpListener, TcpStream};

use crate::connection::SocketConnection;
use crate::msg::request::Request;

pub struct DapCodec {
    listener: TcpListener,
}

pub struct Session {
    pub(crate) connection: SocketConnection,
    next_seq: u64,
}

impl DapCodec {
    pub fn new(listener: TcpListener) -> DapCodec {
        DapCodec { listener }
    }

    pub fn accept(&mut self) -> anyhow::Result<Session> {
        self.listener.set_nonblocking(false)?;
        let (connection, _) = self.listener.accept()?;
        Session::new(connection)
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

    pub(crate) fn next_seq(&mut self) -> u64 {
        let output = self.next_seq;
        self.next_seq += 1;
        output
    }

    pub fn recv_request(&mut self) -> anyhow::Result<Request> {
        self.connection.read_request()
    }
}
