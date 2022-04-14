use std::net::TcpListener;

use dap::adapter::Adapter;

fn main() {
    // this example does not currently have any argument parsing.
    // as such, it can not be ask to listen on a specific port, and instead use a hardcoded value.
    let listener = TcpListener::bind("127.0.0.1:4711").unwrap();

    let mut adapter = Adapter::new(listener);

    loop {
        let mut session = adapter.accept().unwrap();

        loop {
            let msg = session.recv_request().unwrap();
            println!("{msg:#?}");
        }
    }
}
