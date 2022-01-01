use anyhow::{bail, Error};
use serde_json as json;

pub mod event;
pub mod request;
pub mod response;

use event::Event;
use request::Request;
use response::Response;

#[derive(Clone, Debug)]
pub struct Msg {
    seq: u64,
    msg_type: MsgType,
}

#[derive(Clone, Debug)]
pub enum MsgType {
    Request(Request),
    Response(Response),
    Event(Event),
}

pub(crate) fn parse_msg(raw_msg: &str) -> anyhow::Result<Msg> {
    use json::map::Entry;

    let msg: json::Value = json::from_str(raw_msg)?;
    let mut msg = msg.as_object().ok_or(Error::msg("invalid msg"))?.clone();
    let seq = match msg.entry("seq") {
        Entry::Occupied(ref entry) => entry.get().as_u64().ok_or(Error::msg("invalid msg"))?,
        _ => bail!("invalid msg"),
    };

    let msg_type = match msg.entry("type") {
        Entry::Occupied(ref entry) => {
            match entry.get().as_str().ok_or(Error::msg("invalid msg"))? {
                "request" => MsgType::Request(Request::parse(msg)?),
                "response" => MsgType::Response(Response::parse(msg)?),
                "event" => MsgType::Event(Event::parse(msg)?),
                _ => {
                    bail!("invalid msg")
                }
            }
        }
        _ => bail!("invalid msg"),
    };

    let msg = Msg { seq, msg_type };
    Ok(msg)
}
