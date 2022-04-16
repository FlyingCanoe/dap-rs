use anyhow::bail;

use crate::utils::Parse;

pub mod initialize_request;
pub use initialize_request::{InitializeRequest, PathFormat};

#[derive(Debug)]
pub enum Request {
    Initialize(InitializeRequest),
}

impl Request {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<Request> {
        let request_type = String::parse(msg.get("command"))?;

        let request = match request_type.as_str() {
            "initialize" => Request::Initialize(InitializeRequest::parse(msg)?),
            _ => bail!("invalid request"),
        };
        Ok(request)
    }
}
