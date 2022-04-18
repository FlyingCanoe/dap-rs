use std::net::{TcpListener, TcpStream};

use crate::connection::SocketConnection;
use crate::request::{Request, Response};
use crate::utils::ToValue;

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
    next_seq: u64,
}

impl Session {
    fn new(connection: TcpStream) -> Session {
        Session {
            connection: SocketConnection::new(connection),
            next_seq: 1,
        }
    }

    pub(crate) fn next_seq(&mut self) -> u64 {
        let output = self.next_seq;
        self.next_seq += 1;
        output
    }

    pub fn recv_request(&mut self) -> anyhow::Result<Request> {
        let msg_value = self.connection.read_msg()?;
        Request::parse(msg_value)
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

        let msg = json::to_string(&value)?;
        self.connection.send_msg(&msg)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::io::Write;
    use std::net::{TcpListener, TcpStream};
    use std::thread;

    use crate::dap_type::Message;
    use crate::request::{Request, RequestExt};

    use super::{Adapter, Session};

    fn get_init_request_basic() -> json::Value {
        json::json!({
            "command": "initialize",
            "arguments": {
                "adapterID": "mock",
            },
            "type": "request",
            "seq": 1u8
        })
    }

    fn mock_client(input: Vec<u8>) -> Session {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();

        thread::spawn(move || loop {
            let _ = TcpStream::connect(format!("127.0.0.1:{}", port)).map(|mut client| {
                let _ = client.write_all(&input);
            });
        });
        let mut adapter = Adapter::new(listener);
        adapter.accept().unwrap()
    }

    #[test]
    #[should_panic]
    fn recv_request_invalid_utf8_test() {
        // invalid utf-8 example from https://www.cl.cam.ac.uk/~mgk25/ucs/examples/UTF-8-test.txt
        // copyright Markus Kuhn <http://www.cl.cam.ac.uk/~mgk25/> - 2015-08-28 - CC BY 4.0
        let mut input = vec![0xFF, 0xFF];
        input.extend_from_slice(b"\r\n\r\n");

        let mut session = mock_client(input.to_vec());
        session.recv_request().unwrap();
    }

    #[test]
    #[should_panic]
    fn recv_request_unexpected_eoi_test() {
        let mut session = mock_client(vec![]);
        session.recv_request().unwrap();
    }

    #[test]
    #[should_panic]
    fn recv_request_empty_header_test() {
        let input = b"\r\n\r\nbody";

        let mut session = mock_client(input.to_vec());
        session.recv_request().unwrap();
    }

    #[test]
    #[should_panic]
    fn recv_request_invalid_len_test() {
        let input = b"Content-Length: -100\r\n\r\nbody";

        let mut session = mock_client(input.to_vec());
        session.recv_request().unwrap();
    }

    #[test]
    #[should_panic]
    fn recv_request_len_bigger_than_msg_test() {
        let input = b"Content-Length: 5\r\n\r\nbody";

        let mut session = mock_client(input.to_vec());
        session.recv_request().unwrap();
    }

    #[test]
    #[should_panic]
    fn recv_request_invalid_request_test() {
        let msg = json::json!({"command": ""}).to_string();
        let input = format!("Content-Length: {}\r\n\r\n{msg}", msg.len());

        let mut session = mock_client(input.as_bytes().to_vec());
        session.recv_request().unwrap();
    }

    #[test]
    #[should_panic]
    fn recv_request_invalid_request_type_test() {
        let msg = json::json!({"command": 0}).to_string();
        let input = format!("Content-Length: {}\r\n\r\n{msg}", msg.len());

        let mut session = mock_client(input.as_bytes().to_vec());
        session.recv_request().unwrap();
    }

    #[test]
    #[should_panic]
    fn recv_request_body_is_not_json_test() {
        let input = b"Content-Length: 4\r\n\r\nbody";
        let mut session = mock_client(input.to_vec());

        session.recv_request().unwrap();
    }

    #[test]
    fn response_test() {
        let request = get_init_request_basic().to_string();
        let input = format!("Content-Length: {}\r\n\r\n{request}", request.len());

        let mut session = mock_client(input.into_bytes());
        let request = session.recv_request().unwrap();

        match request {
            Request::Initialize(request) => request.respond((), &mut session).unwrap(),
        }
    }

    #[test]
    fn response_with_error_test() {
        let request = get_init_request_basic().to_string();
        let input = format!("Content-Length: {}\r\n\r\n{request}", request.len());

        let mut session = mock_client(input.into_bytes());
        let request = session.recv_request().unwrap();

        match request {
            Request::Initialize(request) => request
                .respond_with_error(Some("error".to_string()), None, &mut session)
                .unwrap(),
        }
    }

    #[test]
    fn response_with_structured_error_test() {
        let request = get_init_request_basic().to_string();
        let input = format!("Content-Length: {}\r\n\r\n{request}", request.len());

        let mut session = mock_client(input.into_bytes());
        let request = session.recv_request().unwrap();

        let var: HashMap<String, String> = vec![
            ("test_key".to_string(), "test_value".to_string()),
            ("test_key2".to_string(), "test_value2".to_string()),
        ]
        .into_iter()
        .collect();

        let msg = Message {
            url: Some("http://example.com".to_string()),
            id: 0,
            format: "".to_string(),
            variables: Some(var),
            show_user: Some(true),
            url_label: Some("test".to_string()),
            send_telemetry: Some(false),
        };

        match request {
            Request::Initialize(request) => request
                .respond_with_error(Some("error".to_string()), Some(msg), &mut session)
                .unwrap(),
        };
    }
}
