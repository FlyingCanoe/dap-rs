use std::net::TcpListener;

use dap::connection::SocketConnection;

fn main() {
    // this example does not currently have any argument parsing.
    // as such, it can not be ask to listen on a specific port, and instead use a hardcoded value.
    let listener = TcpListener::bind("127.0.0.1:4711").unwrap();

    for connection in listener.incoming() {
        let mut connection = SocketConnection::new(connection.unwrap());

        loop {
            let msg = connection.read_raw_msg().unwrap();
            println!("{msg}");
        }
    }
}
