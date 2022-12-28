use std::net;

use dap::{
    codec::DapCodec,
    msg::{
        dap_type::{thread::Thread, Capabilities},
        event::{Event, InitializedEvent},
        request::{
            AcknowledgementResponse, DisconnectResponse, InitializeResponse, PauseResponse,
            Request, Response, ResponseType, SetBreakpointsResponse,
            SetExceptionBreakpointsResponse, ThreadsResponse,
        },
        MsgType,
    },
    utils::ToValue,
};

fn main() {
    let listener = net::TcpListener::bind("127.0.0.1:4711").unwrap();
    let codec = DapCodec::new(listener);

    let cap = Capabilities::default();

    let init_event = Event::Initialized(InitializedEvent {});
    let init_event = MsgType::Event(init_event);

    loop {
        let mut session = codec.accept().unwrap();

        loop {
            let msg = session.recv().unwrap();
            println!(
                "{}",
                serde_json::to_string_pretty(&msg.clone().to_value().unwrap()).unwrap()
            );

            let MsgType::Request(request) = msg.msg_type else {panic!()};

            let response = match request {
                Request::Launch(request) => ResponseType::Acknowledgement(
                    AcknowledgementResponse::new(request.command().to_string()),
                ),
                Request::SetExceptionBreakpoints(_) => {
                    let response = SetExceptionBreakpointsResponse { breakpoints: None };
                    ResponseType::SetExceptionBreakpoints(response)
                }
                Request::Threads(_) => {
                    let response = ThreadsResponse {
                        threads: vec![Thread {
                            id: 0,
                            name: "main".to_string(),
                        }],
                    };
                    ResponseType::Threads(response)
                }
                Request::SetBreakpoints(_) => {
                    let response = SetBreakpointsResponse {
                        breakpoints: vec![],
                    };
                    ResponseType::SetBreakpoints(response)
                }
                Request::Pause(_) => {
                    let response = PauseResponse {};

                    ResponseType::Pause(response)
                }
                Request::Disconnect(_) => {
                    let response = DisconnectResponse {};
                    let response = ResponseType::Disconnect(response);

                    session
                        .send(MsgType::Response(Response {
                            request_seq: msg.seq,
                            response_type: response,
                        }))
                        .unwrap();

                    break;
                }
                Request::Initialize(_) => {
                    let response = InitializeResponse {
                        capabilities: Some(cap.clone()),
                    };
                    let response = ResponseType::Initialize(response);
                    let response = Response {
                        request_seq: msg.seq,
                        response_type: response,
                    };

                    session.send(MsgType::Response(response)).unwrap();

                    session.send(init_event.clone()).unwrap();
                    continue;
                }
                _ => panic!(),
            };

            session
                .send(MsgType::Response(Response {
                    request_seq: msg.seq,
                    response_type: response,
                }))
                .unwrap();
        }
    }
}
