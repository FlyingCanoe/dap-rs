use std::net::{TcpListener, TcpStream};

use crate::connection::SocketConnection;
use crate::msg::{Msg, MsgType};

pub struct DapCodec {
    listener: TcpListener,
}

pub struct Session {
    connection: SocketConnection,
    next_seq: u64,
}

impl DapCodec {
    pub fn new(listener: TcpListener) -> DapCodec {
        DapCodec { listener }
    }

    pub fn accept(&self) -> anyhow::Result<Session> {
        self.listener.set_nonblocking(false)?;

        let (connection, _) = self.listener.accept()?;
        let session = Session::new(connection)?;
        Ok(session)
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

    pub fn recv(&mut self) -> anyhow::Result<Msg> {
        self.connection.read_msg()
    }

    pub fn try_recv(&mut self) -> anyhow::Result<Option<Msg>> {
        self.connection.try_read_msg()
    }

    pub fn send(&mut self, msg: MsgType) -> Result<(), anyhow::Error> {
        let seq = self.next_seq();
        self.connection.send_msg(Msg { seq, msg_type: msg })
    }
}
