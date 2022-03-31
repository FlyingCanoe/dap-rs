use crate::msg::request::ResponseType;
use crate::utils::ToValue;
use crate::{codec::Session, msg::dap_type::Capabilities};

use super::{ErrorResponse, RequestExt, Response};

dap_type_enum!(
    /// Determines in what format paths are specified. The default is 'path', which is the native format.
    PathFormat {
        Other,
        Path | "path",
        Uri | "uri",
    }
);

request!(
    /// The 'initialize' request is sent as the first request from the client to the debug adapter
    /// in order to configure it with client capabilities and to retrieve capabilities from the debug adapter.
    /// Until the debug adapter has responded to with an 'initialize' response, the client must not send any additional requests or events to the debug adapter.
    /// In addition the debug adapter is not allowed to send any requests or events to the client until it has responded with an 'initialize' response.
    /// The 'initialize' request may only be sent once.
    InitializeRequest | "initialize" {
        /// The ISO-639 locale of the (frontend) client using this adapter, e.g. en-US or de-CH.
        locale | "locale": Option<String>,
        /// Client supports the optional type attribute for variables.
        supports_variable_type | "supportsVariableType": Option<bool>,
        /// Client supports memory references.
        supports_memory_references | "supportsMemoryReferences": Option<bool>,
        /// The ID of the (frontend) client using this adapter.
        client_id | "clientID": Option<String>,
        /// If true all line numbers are 1-based (default).
        lines_start_at1 | "linesStartAt1": Option<bool>,
        /// Client supports the paging of variables.
        supports_variable_paging | "supportsVariablePaging": Option<bool>,
        /// The ID of the debug adapter.
        adapter_id | "adapterID": String,
        /// Client supports progress reporting.
        supports_progress_reporting | "supportsProgressReporting": Option<bool>,
        /// Client supports the runInTerminal request.
        supports_run_in_terminal_request | "supportsRunInTerminalRequest": Option<bool>,
        /// Client supports the invalidated event.
        supports_invalidated_event | "supportsInvalidatedEvent": Option<bool>,
        /// Determines in what format paths are specified. The default is 'path', which is the native format.
        path_format | "pathFormat": Option<PathFormat>,
        /// If true all column numbers are 1-based (default).
        columns_start_at1 | "columnsStartAt1": Option<bool>,
        /// Client supports the memory event.
        supports_memory_event | "supportsMemoryEvent": Option<bool>,
        /// The human readable name of the (frontend) client using this adapter.
        client_name | "clientName": Option<String>,
    }
);

/// Response to 'initialize' request.
#[derive(Debug, Clone)]
pub struct InitializeResponse {
    pub capabilities: Option<Capabilities>,
}

impl ToValue for InitializeResponse {
    fn to_value(self) -> Option<serde_json::Value> {
        let mut msg = serde_json::Map::new();

        msg.insert("type".to_string(), "response".into());
        msg.insert("success".to_string(), true.into());
        msg.insert("command".to_string(), "initialize".into());

        if let Some(cap) = self.capabilities.to_value() {
            msg.insert("body".to_string(), cap);
        }
        Some(msg.into())
    }
}

impl RequestExt for InitializeRequest {
    type Response = InitializeResponse;
    fn respond(
        self,
        response: Result<InitializeResponse, ErrorResponse>,
        session: &mut Session,
    ) -> Result<(), anyhow::Error> {
        let response_type = match response {
            Ok(response) => ResponseType::Initialize(response),
            Err(err) => ResponseType::Error(err),
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
