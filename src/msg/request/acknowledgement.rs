use serde_json as json;

use crate::utils::ToValue;

#[derive(Clone, Debug)]
pub(crate) struct AcknowledgementResponse {
    pub(crate) command: String,
}

impl AcknowledgementResponse {
    pub(crate) fn new(command: String) -> AcknowledgementResponse {
        AcknowledgementResponse { command }
    }
}

impl ToValue for AcknowledgementResponse {
    fn to_value(self) -> Option<json::Value> {
        let mut msg = json::Map::new();

        msg.insert("type".to_string(), "response".into());
        msg.insert("command".to_string(), self.command.into());
        msg.insert("success".to_string(), true.into());

        Some(msg.into())
    }
}
