use serde_json as json;

use crate::msg::dap_type::message::Message;
use crate::utils::ToValue;

#[derive(Clone, Debug)]
/// On error (whenever 'success' is false), the body can provide more details.
pub struct ErrorResponse {
    /// Contains the raw error in short form if 'success' is false.
    /// This raw error might be interpreted by the frontend and is not shown in the
    /// UI.
    /// Some predefined values exist.
    /// Values:
    /// - 'cancelled': request was cancelled.
    /// - etc.
    pub message: Option<String>,
    /// An optional, structured error message.
    pub error: Option<Message>,
    pub(crate) command: Option<String>,
}

impl ErrorResponse {
    pub fn new(message: Option<String>, error: Option<Message>) -> ErrorResponse {
        ErrorResponse {
            message,
            error,
            command: None,
        }
    }

    pub(crate) fn with_command(mut self, command: String) -> ErrorResponse {
        self.command = Some(command);
        self
    }
}

impl ToValue for ErrorResponse {
    fn to_value(self) -> Option<json::Value> {
        debug_assert!(self.command.is_some());

        let mut msg = json::Map::new();
        let mut body = json::Map::new();

        msg.insert("type".to_string(), "response".into());
        msg.insert("command".to_string(), self.command?.into());
        msg.insert("success".to_string(), false.into());

        if let Some(value) = self.message.to_value() {
            msg.insert("message".to_string(), value);
        }
        if let Some(value) = self.error.to_value() {
            body.insert("error".to_string(), value);
        }

        msg.insert("body".to_string(), body.into());
        Some(msg.into())
    }
}
