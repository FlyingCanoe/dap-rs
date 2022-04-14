use std::net::{TcpListener, TcpStream};

use crate::connection::SocketConnection;
use crate::request::Request;

pub struct Adapter {
    listener: TcpListener,
}

impl Adapter {
    pub fn new(listener: TcpListener) -> Adapter {
        Adapter { listener }
    }

    pub fn accept(&mut self) -> anyhow::Result<Session> {
        let (connection, _) = self.listener.accept()?;
        let session = Session::new(connection);
        Ok(session)
    }
}

pub struct Session {
    connection: SocketConnection,
}

impl Session {
    fn new(connection: TcpStream) -> Session {
        Session {
            connection: SocketConnection::new(connection),
        }
    }

    pub fn recv_request(&mut self) -> anyhow::Result<Request> {
        let msg_value = self.connection.read_msg()?;
        Request::parse(msg_value)
    }
}

#[cfg(test)]
mod test {
    use std::io::Write;
    use std::net::{TcpListener, TcpStream};
    use std::thread;

    use super::Adapter;

    fn mock_client(input: Vec<u8>) -> TcpListener {
        let adapter = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = adapter.local_addr().unwrap().port();

        thread::spawn(move || loop {
            let _ = TcpStream::connect(format!("127.0.0.1:{}", port)).map(|mut client| {
                let _ = client.write_all(&input);
            });
        });
        adapter
    }

    #[test]
    #[should_panic]
    fn recv_request_invalid_utf8_test() {
        // invalid utf-8 example from https://www.cl.cam.ac.uk/~mgk25/ucs/examples/UTF-8-test.txt
        // copyright Markus Kuhn <http://www.cl.cam.ac.uk/~mgk25/> - 2015-08-28 - CC BY 4.0
        let mut input = vec![0xFF, 0xFF];
        input.extend_from_slice(b"\r\n\r\n");

        let socket = mock_client(input);

        let mut adapter = Adapter::new(socket);
        let mut session = adapter.accept().unwrap();
        session.recv_request().unwrap();
    }

    #[test]
    #[should_panic]
    fn recv_request_unexpected_eoi_test() {
        let socket = mock_client(vec![]);
        let mut adapter = Adapter::new(socket);
        let mut session = adapter.accept().unwrap();
        session.recv_request().unwrap();
    }

    #[test]
    #[should_panic]
    fn recv_request_empty_header_test() {
        let input = b"\r\n\r\nbody";

        let socket = mock_client(input.to_vec());

        let mut adapter = Adapter::new(socket);
        let mut session = adapter.accept().unwrap();
        session.recv_request().unwrap();
    }

    #[test]
    #[should_panic]
    fn recv_request_invalid_len_test() {
        let input = b"Content-Length: -100\r\n\r\nbody";

        let socket = mock_client(input.to_vec());

        let mut adapter = Adapter::new(socket);
        let mut session = adapter.accept().unwrap();

        session.recv_request().unwrap();
    }

    #[test]
    #[should_panic]
    fn recv_request_len_bigger_than_msg_test() {
        let input = b"Content-Length: 5\r\n\r\nbody";

        let socket = mock_client(input.to_vec());

        let mut adapter = Adapter::new(socket);
        let mut session = adapter.accept().unwrap();

        session.recv_request().unwrap();
    }

    #[test]
    #[should_panic]
    fn recv_request_invalid_request_test() {
        let msg = json::json!({"command": ""}).to_string();
        let input = format!("Content-Length: {}\r\n\r\n{msg}", msg.len());

        let socket = mock_client(input.as_bytes().to_vec());

        let mut adapter = Adapter::new(socket);
        let mut session = adapter.accept().unwrap();

        session.recv_request().unwrap();
    }

    #[test]
    #[should_panic]
    fn recv_request_invalid_request_type_test() {
        let msg = json::json!({"command": 0}).to_string();
        let input = format!("Content-Length: {}\r\n\r\n{msg}", msg.len());

        let socket = mock_client(input.as_bytes().to_vec());

        let mut adapter = Adapter::new(socket);
        let mut session = adapter.accept().unwrap();

        session.recv_request().unwrap();
    }

    #[test]
    #[should_panic]
    fn recv_request_body_is_not_json_test() {
        let input = b"Content-Length: 4\r\n\r\nbody";

        let socket = mock_client(input.to_vec());

        let mut adapter = Adapter::new(socket);
        let mut session = adapter.accept().unwrap();

        session.recv_request().unwrap();
    }
}
