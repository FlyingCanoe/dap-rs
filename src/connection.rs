use anyhow::{bail, Result};
use bstr::{BString, ByteSlice, B};
use std::io::Read;
use std::net::TcpStream;

struct SocketConnection {
    inner_connection: TcpStream,
    buf: BString,
}

impl SocketConnection {
    fn new(input: TcpStream) -> Result<SocketConnection> {
        input.set_nodelay(true)?;
        input.set_nonblocking(true)?;
        Ok(SocketConnection {
            inner_connection: input,
            buf: BString::from(String::new()),
        })
    }

    fn try_read_line(&mut self) -> Result<Option<String>> {
        use std::io::ErrorKind;

        let mut buf = [0; 4096];
        while !self.buf.contains_str("\n") {
            match self.inner_connection.read(&mut buf) {
                Ok(read_size) => self.buf.extend_from_slice(&buf[0..read_size]),
                Err(err) if err.kind() == ErrorKind::WouldBlock => return Ok(None),
                Err(err) => bail!(err),
            }
        }

        let mut iter = self.buf.split_str(B("\n"));
        let line = iter.next().unwrap().to_vec();
        self.buf = iter.flatten().copied().collect();
        let line = String::from_utf8(line)?;
        return Ok(Some(line));
    }

    fn read_exact(&mut self) {}
}
