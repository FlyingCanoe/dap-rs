use std::net;

use dap::{
    codec::DapCodec,
    msg::{
        request::{AcknowledgementResponse, Response, ResponseType},
        MsgType,
    },
    utils::ToValue,
};

fn main() {
    let listener = net::TcpListener::bind("127.0.0.1:4711").unwrap();
    let codec = DapCodec::new(listener);

    let mut session = codec.accept().unwrap();

    let msg = session.recv().unwrap();

    let MsgType::Request(request) = msg.msg_type else {panic!()};

    let response = AcknowledgementResponse::new(request.command().to_string());
    let response = ResponseType::Acknowledgement(response);

    let response = Response {
        request_seq: msg.seq,
        response_type: response,
    };

    session.send(MsgType::Response(response)).unwrap();

    let msg = session.recv().unwrap();
    println!(
        "{}",
        serde_json::to_string_pretty(&msg.to_value().unwrap()).unwrap()
    )
}
