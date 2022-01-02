use anyhow::{bail, Error};
use serde_json as json;

pub mod dap_type;
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
    let msg: json::Value = json::from_str(raw_msg)?;
    let seq = msg
        .get("seq")
        .map_or(None, json::Value::as_u64)
        .ok_or(Error::msg("invalid msg"))?;

    let msg_type = msg
        .get("type")
        .map_or(None, json::Value::as_str)
        .ok_or(Error::msg("invalid msg"))?;

    let msg_type = match msg_type {
        "request" => MsgType::Request(Request::parse(msg)?),
        "response" => MsgType::Response(Response::parse(msg)?),
        "event" => MsgType::Event(Event::parse(msg)?),
        _ => {
            bail!("invalid msg")
        }
    };

    let msg = Msg { seq, msg_type };
    Ok(msg)
}
