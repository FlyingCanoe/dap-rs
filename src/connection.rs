use std::io::Read;
use std::io::{self, Write};
use std::net::TcpStream;

use anyhow::{bail, Error};
use bstr::{BString, ByteSlice, B};
use serde_json as Json;

use crate::msg::request::{Request, Response};
use crate::utils::ToValue;

#[derive(Debug)]
pub struct SocketConnection {
    inner_connection: TcpStream,
    buf: BString,
}

impl SocketConnection {
    pub fn new(raw_connection: TcpStream) -> anyhow::Result<SocketConnection> {
        raw_connection.set_nodelay(true)?;
        raw_connection.set_nonblocking(true)?;
        Ok(SocketConnection {
            inner_connection: raw_connection,
            buf: BString::from(String::new()),
        })
    }

    fn read(&mut self) -> Result<(), io::Error> {
        let mut buf = [0; 4096];
        match self.inner_connection.read(&mut buf) {
            Ok(read_size) => self.buf.extend_from_slice(&buf[0..read_size]),
            Err(err) => return Err(err),
        }
        Ok(())
    }

    fn try_read_header(&mut self) -> anyhow::Result<Option<String>> {
        use std::io::ErrorKind;

        while !self.buf.contains_str("\r\n\r\n") {
            match self.read() {
                Ok(_) => {}
                Err(err) if err.kind() == ErrorKind::WouldBlock => return Ok(None),
                Err(err) => bail!(err),
            }
        }

        let mut iter = self.buf.split_str(B("\r\n"));
        let line = iter.next().unwrap().to_vec();
        self.buf = iter.flatten().copied().collect();
        let line = String::from_utf8(line)?;
        return Ok(Some(line));
    }

    fn read_exact(&mut self, len: usize) -> anyhow::Result<Vec<u8>> {
        use std::io::ErrorKind;

        while self.buf.len() < len {
            match self.read() {
                Ok(_) => {}
                Err(err) if err.kind() == ErrorKind::WouldBlock => {}
                Err(err) => bail!(err),
            }
        }

        let (output, rem) = self.buf.split_at(len);
        let output = output.to_owned();
        self.buf = BString::from(rem);
        return Ok(output);
    }

    pub fn try_parse_header(&mut self) -> anyhow::Result<Option<usize>> {
        if let Some(header) = self.try_read_header()? {
            let content_length = header
                .lines()
                .find(|line| line.starts_with("Content-Length"))
                .ok_or(Error::msg("bad header"))?;

            let len: usize = content_length
                .split(":")
                .nth(1)
                .ok_or(Error::msg("bad header"))?
                .trim()
                .parse()?;

            Ok(Some(len))
        } else {
            Ok(None)
        }
    }

    fn try_read_raw_msg(&mut self) -> anyhow::Result<Option<String>> {
        if let Some(msg_size) = self.try_parse_header()? {
            let msg = self.read_exact(msg_size)?;
            let msg = String::from_utf8(msg)?;
            Ok(Some(msg))
        } else {
            Ok(None)
        }
    }

    #[allow(dead_code)]
    pub(crate) fn try_read_request(&mut self) -> anyhow::Result<Option<Request>> {
        if let Some(raw_msg) = self.try_read_raw_msg()? {
            let msg = Request::parse(&raw_msg)?;
            Ok(Some(msg))
        } else {
            Ok(None)
        }
    }

    pub(crate) fn read_request(&mut self) -> anyhow::Result<Request> {
        self.inner_connection.set_nonblocking(false)?;
        let raw_request = self.try_read_raw_msg()?.unwrap();
        let request = Request::parse(&raw_request)?;
        self.inner_connection.set_nonblocking(true)?;
        Ok(request)
    }

    pub(crate) fn send_response(&mut self, response: Response) -> anyhow::Result<()> {
        let msg = Json::to_string_pretty(&response.to_value())?;
        let msg_header = format!("Content-Length: {}\r\n\r\n", msg.len());

        println!("send:{msg}");

        self.inner_connection.write_all(msg_header.as_bytes())?;
        self.inner_connection.write_all(msg.as_bytes())?;
        Ok(())
    }
}
