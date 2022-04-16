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

#[cfg(test)]
mod test {
    use crate::request::Request;

    #[test]
    fn initialize_request_parse_test() {
        let value = json::json!(
            {
                "seq": 1,
                "type": "request",
                "command": "initialize",
                "arguments": {
                    "clientID": "client_id",
                    "clientName": "client_name",
                    "adapterID": "adapter_id",
                    "locale": "en-us",
                    "linesStartAt1": true,
                    "columnsStartAt1": true,
                    "pathFormat": "path",
                    "supportsVariableType": true,
                    "supportsVariablePaging": true,
                    "supportsRunInTerminalRequest": true,
                    "supportsMemoryReferences": true,
                    "supportsProgressReporting": true,
                    "supportsInvalidatedEvent": true,
                    "supportsMemoryEvent": true,
                }
            }
        );

        Request::parse(value).unwrap();
    }

    #[test]
    #[should_panic]
    fn initialize_request_no_argument_test() {
        let value = json::json!(
            {
                "seq": 1,
                "type": "request",
                "command": "initialize",
            }
        );

        Request::parse(value).unwrap();
    }

    #[test]
    #[should_panic]
    fn initialize_request_no_empty_argument_test() {
        let value = json::json!(
            {
                "seq": 1,
                "type": "request",
                "command": "initialize",
                "arguments": {}
            }
        );

        Request::parse(value).unwrap();
    }

    #[test]
    fn initialize_request_other_path_format_test() {
        let value = json::json!(
            {
                "seq": 1,
                "type": "request",
                "command": "initialize",
                "arguments": {
                    "clientID": "client_id",
                    "clientName": "client_name",
                    "adapterID": "adapter_id",
                    "locale": "en-us",
                    "linesStartAt1": true,
                    "columnsStartAt1": true,
                    "pathFormat": "some other path format",
                    "supportsVariableType": true,
                    "supportsVariablePaging": true,
                    "supportsRunInTerminalRequest": true,
                    "supportsMemoryReferences": true,
                    "supportsProgressReporting": true,
                    "supportsInvalidatedEvent": true,
                    "supportsMemoryEvent": true,
                }
            }
        );

        Request::parse(value).unwrap();
    }
}
