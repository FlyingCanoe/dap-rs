use std::net::{TcpListener, TcpStream};

use serde_json::to_string_pretty;

use crate::connection::SocketConnection;
use crate::msg::event::Event;
use crate::msg::request::Request;
use crate::msg::request::Response;
use crate::utils::ToValue;

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

    pub fn try_recv_request(&mut self) -> anyhow::Result<Option<Request>> {
        self.connection.try_read_request()
    }

    pub fn send_event(&mut self, event: Event) -> anyhow::Result<()> {
        let mut value = event.to_value().unwrap();
        let map = value.as_object_mut().unwrap();

        map.insert("seq".to_string(), self.next_seq().into());
        map.insert("type".to_string(), "event".into());

        let msg = to_string_pretty(&value)?;
        self.connection.send_msg(&msg)
    }

    pub(crate) fn send_response(
        &mut self,
        response: Response,
        request_seq: u64,
    ) -> anyhow::Result<()> {
        let mut value = response.to_value().unwrap();
        let map = value.as_object_mut().unwrap();

        map.insert("seq".to_string(), self.next_seq().into());
        map.insert("type".to_string(), "response".into());
        map.insert("request_seq".to_string(), request_seq.into());

        let msg = to_string_pretty(&value)?;
        self.connection.send_msg(&msg)
    }
}
