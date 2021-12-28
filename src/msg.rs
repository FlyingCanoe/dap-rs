pub mod event;
pub mod request;
pub mod response;

use event::Event;
use request::Request;
use response::Response;

pub struct Msg {
    seq: usize,
}

pub enum MsgType {
    Request(Request),
    Response(Response),
    Event(Event),
}
