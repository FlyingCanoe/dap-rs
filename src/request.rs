use anyhow::bail;

use crate::adapter::Session;
use crate::dap_type::message::Message;
use crate::utils::Parse;
use crate::utils::ToValue;

pub(crate) mod acknowledgement_response;
pub mod error_response;
pub mod initialize_request;

use acknowledgement_response::AcknowledgementResponse;
use error_response::ErrorResponse;
use initialize_request::InitializeResponse;
pub use initialize_request::{InitializeRequest, PathFormat};

pub trait RequestExt {
    type Response;

    fn respond(self, response: Self::Response, session: &mut Session) -> Result<(), anyhow::Error>;

    fn respond_with_error(
        self,
        message: Option<String>,
        error: Option<Message>,
        session: &mut Session,
    ) -> Result<(), anyhow::Error>;
}

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

#[derive(Clone, Debug)]
pub(crate) enum Response {
    Error(ErrorResponse),
    Acknowledgement(AcknowledgementResponse),
    Initialize(InitializeResponse),
}

impl ToValue for Response {
    fn to_value(self) -> Option<json::Value> {
        match self {
            Response::Error(response) => response.to_value(),
            Response::Acknowledgement(response) => response.to_value(),
            Response::Initialize(response) => response.to_value(),
        }
    }
}

impl From<AcknowledgementResponse> for Response {
    fn from(response: AcknowledgementResponse) -> Self {
        Self::Acknowledgement(response)
    }
}

impl From<ErrorResponse> for Response {
    fn from(response: ErrorResponse) -> Self {
        Self::Error(response)
    }
}
