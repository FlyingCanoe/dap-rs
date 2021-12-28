use std::net;

use dap::connection::SocketConnection;

fn main() {
    let listener = net::TcpListener::bind("127.0.0.1:4710").unwrap();
    for raw_connection in listener.incoming() {
        let raw_connection = raw_connection.unwrap();
        let mut connection = SocketConnection::new(raw_connection).unwrap();
        loop {
            match connection.try_read_msg().unwrap() {
                Some(msg) => {
                    println!("{}", msg)
                }
                None => {}
            }
        }
    }
}
