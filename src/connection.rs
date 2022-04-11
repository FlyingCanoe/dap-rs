use std::io::Read;
use std::{io, net::TcpStream};

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

    pub fn read_header(&mut self) -> anyhow::Result<String> {
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
}

#[cfg(test)]
mod test {
    use std::io::{ErrorKind, Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::{io, thread};

    use super::SocketConnection;

    fn mock_client(input: Vec<u8>) -> TcpStream {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();

        let join_handle = thread::spawn(move || listener.accept().unwrap());

        let mut client = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
        let (adapter, _) = join_handle.join().unwrap();

        thread::spawn(move || {
            client.write_all(&input).unwrap();
        });
        adapter
    }

    #[test]
    fn read_header_test() {
        let input = "header\r\n\r\nbody".as_bytes();
        let socket = mock_client(input.to_vec());

        let mut connection = SocketConnection::new(socket);
        let output = connection.read_header().unwrap();

        assert_eq!(output.as_str(), "header")
    }

    #[test]
    #[should_panic]
    fn read_header_invalid_utf8_test() {
        // invalid utf-8 example from https://www.cl.cam.ac.uk/~mgk25/ucs/examples/UTF-8-test.txt
        // copyright Markus Kuhn <http://www.cl.cam.ac.uk/~mgk25/> - 2015-08-28 - CC BY 4.0
        let mut input = vec![0xFF, 0xFF];
        input.extend_from_slice(b"\r\n\r\n");

        let socket = mock_client(input.to_vec());
        let mut connection = SocketConnection::new(socket);
        connection.read_header().unwrap();
    }

    #[test]
    #[should_panic]
    fn read_header_unexpected_eoi_test() {
        let input = "".as_bytes();

        let socket = mock_client(input.to_vec());
        let mut connection = SocketConnection::new(socket);
        connection.read_header().unwrap();
    }
}
