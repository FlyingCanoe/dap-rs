use std::collections::HashMap;

use anyhow::Error;
use serde_json as json;

use crate::utils::Parse;

use super::{AcknowledgementResponse, RequestExt, Response, ResponseType};

#[derive(Debug, Clone)]
/// This launch request is sent from the client to the debug adapter to start the debuggee with or without debugging (if 'noDebug' is true).
/// Since launching is debugger/runtime specific, the arguments for this request are not part of this specification.
pub struct LaunchRequest {
    seq: u64,

    /// If noDebug is true the launch request should launch the program without enabling debugging.
    pub no_debug: Option<bool>,

    /// Optional data from the previous, restarted session.
    /// The data is sent as the 'restart' attribute of the 'terminated' event.
    /// The client should leave the data intact.
    pub restart: Option<json::Value>,

    /// Since launching is debugger/runtime specific, the arguments for this request are not part of this specification.
    pub additional_data: HashMap<String, json::Value>,
}

impl LaunchRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<LaunchRequest> {
        let args = msg
            .get("arguments")
            .ok_or(anyhow::Error::msg("invalid request"))?;

        let seq = msg
            .get("seq")
            .ok_or(Error::msg("parsing error"))?
            .as_u64()
            .ok_or(Error::msg("parsing error"))?;

        let no_debug = Option::<bool>::parse(args.get("noDebug"))?;
        let restart = Option::<json::Value>::parse(args.get("__restart"))?;

        let additional_data: HashMap<_, _> = args
            .as_object()
            .cloned()
            .ok_or(Error::msg("parsing error"))?
            .into_iter()
            .filter(|(key, _)| key != "noDebug" && key != "__restart")
            .collect();

        let request = LaunchRequest {
            seq,
            no_debug,
            restart,
            additional_data,
        };
        Ok(request)
    }

    pub(crate) const fn command(&self) -> &'static str {
        "launch"
    }
}

impl RequestExt for LaunchRequest {
    type Response = ();

    fn respond(
        self,
        response: Result<Self::Response, super::ErrorResponse>,
        session: &mut crate::codec::Session,
    ) -> Result<(), anyhow::Error> {
        let response_type = match response {
            Ok(_) => ResponseType::from(AcknowledgementResponse::new("launch".to_string())),
            Err(err) => ResponseType::from(err),
        };

        let seq = session.next_seq();
        session.connection.send_response(Response {
            seq,
            request_seq: self.seq,
            response_type,
        })?;
        Ok(())
    }
}

impl crate::utils::ToValue for LaunchRequest {
    fn to_value(self) -> Option<json::Value> {
        let mut msg = json::Map::new();
        let mut arguments = json::Map::new();

        msg.insert("type".to_string(), "response".into());
        msg.insert("success".to_string(), true.into());
        msg.insert("command".to_string(), self.command().into());

        self.no_debug
            .to_value()
            .map(|value| arguments.insert("noDebug".to_string(), value));

        self.restart
            .to_value()
            .map(|value| arguments.insert("__restart".to_string(), value));

        for (key, value) in self.additional_data {
            arguments.insert(key, value);
        }

        msg.insert("arguments".to_string(), arguments.into());
        Some(msg.into())
    }
}
