use anyhow::bail;

use crate::utils::Parse;

#[derive(Debug)]
pub enum Request {}

impl Request {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<Request> {
        let request_type = String::parse(msg.get("command"))?;

        match request_type.as_str() {
            _ => bail!("invalid request"),
        };
    }
}
