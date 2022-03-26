use std::collections::HashMap;

use anyhow::Error;
use serde_json as json;

use crate::utils::Parse;
/// The attach request is sent from the client to the debug adapter to attach to a debuggee that is already running.
/// Since attaching is debugger/runtime specific, the arguments for this request are not part of this specification.
#[derive(Debug, Clone)]
pub struct AttachRequest {
    /// Optional data from the previous, restarted session.
    /// The data is sent as the 'restart' attribute of the 'terminated' event.
    /// The client should leave the data intact.
    pub restart: Option<json::Value>,

    pub additional_data: HashMap<String, json::Value>,
}

impl AttachRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<AttachRequest> {
        let args = msg
            .get("arguments")
            .ok_or(anyhow::Error::msg("invalid request"))?;

        let restart = Option::<json::Value>::parse(args.get("__restart"))?;

        let additional_data: HashMap<_, _> = args
            .as_object()
            .cloned()
            .ok_or(Error::msg("parsing error"))?
            .into_iter()
            .filter(|(key, _)| key != "__restart")
            .collect();

        let request = AttachRequest {
            restart,
            additional_data,
        };
        Ok(request)
    }

    pub(crate) const fn command(&self) -> &'static str {
        "attach"
    }
}

impl crate::utils::ToValue for AttachRequest {
    fn to_value(self) -> Option<json::Value> {
        let mut msg = json::Map::new();
        let mut arguments = json::Map::new();

        msg.insert("type".to_string(), "response".into());
        msg.insert("success".to_string(), true.into());
        msg.insert("command".to_string(), self.command().into());

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

response!(
    /// Response to 'attach' request. This is just an acknowledgement, so no body field is required.
    AttachResponse | "attach" {}
);
