use std::io::Read;
use std::{io, net::TcpStream};

use anyhow::Error;
use bstr::{BString, ByteSlice};

/// A connection betweens the adapter and the client.
/// This struct handle the raw io..
pub struct SocketConnection {
    socket: TcpStream,
    buf: BString,
}

impl SocketConnection {
    pub fn new(input: TcpStream) -> SocketConnection {
        SocketConnection {
            socket: input,
            buf: BString::from(String::new()),
        }
    }

    fn read(&mut self) -> Result<(), io::Error> {
        let mut buf = [0; 4096];
        let read_size = self.socket.read(&mut buf)?;

        // if the read_size it zero, it mean that the eof/eoi was reach.
        if read_size == 0 {
            Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "unexpected end of input",
            ))
        } else {
            self.buf.extend_from_slice(&buf[0..read_size]);
            Ok(())
        }
    }

    fn read_header(&mut self) -> anyhow::Result<String> {
        // block until the buffer contain a header
        while !self.buf.contains_str("\r\n\r\n") {
            self.read()?;
        }

        // extract the header
        let mut iter = self.buf.split_str(b"\r\n\r\n");
        let header = iter.next().unwrap().to_owned();

        // put the rest into the buffer
        self.buf = iter.collect();

        // convert the bytes to a String
        let header = String::from_utf8(header)?;
        return Ok(header);
    }

    /// Find the length of the msg body.
    /// The standard currently only specify one non optional field: the Content-Length field.
    /// As such, this function return the value of this field.
    fn parse_header(&mut self) -> anyhow::Result<usize> {
        let header = self.read_header()?;

        // find the Content-Length header field
        let msg_len_field = header
            .lines()
            .find(|line| line.starts_with("Content-Length: "))
            .ok_or(Error::msg("bad msg"))?;

        // parse the Content-Length header field
        let (_, msg_len) = msg_len_field.trim().split_once("Content-Length: ").unwrap();
        let msg_len = msg_len.parse()?;

        Ok(msg_len)
    }

    fn read_raw_msg(&mut self) -> anyhow::Result<String> {
        let msg_len = self.parse_header()?;

        // read the msg
        while self.buf.len() < msg_len {
            self.read()?;
        }

        // extract the msg
        let (msg, rem) = self.buf.split_at(msg_len);
        let msg = msg.to_owned(); // this is needed to please the barrow checker

        // put the rest into the buffer
        self.buf = BString::from(rem);

        // convert the bytes into a string
        let msg = String::from_utf8(msg).unwrap();
        Ok(msg)
    }

    pub fn read_msg(&mut self) -> anyhow::Result<json::Value> {
        let raw_msg = self.read_raw_msg()?;
        Ok(json::from_str(&raw_msg)?)
    }
}
