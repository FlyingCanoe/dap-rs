use crate::msg::dap_type::PathFormat;

request!(
    InitializeRequest {
        optional_args = false;
        u64 {},
        Option<u64> {},
        Option<Vec<u64>> {},
        Option<bool> {
            /// If true all line numbers are 1-based (default).
            lines_start_at_1: "linesStartAt1",
            /// If true all column numbers are 1-based (default).
            columns_start_at_1: "columnsStartAt1",
            /// Client supports the optional type attribute for variables.
            supports_variable_type: "supportsVariableType",
            /// Client supports the paging of variables.
            supports_variable_paging: "supportsVariablePaging",
            /// Client supports the runInTerminal request.
            supports_run_in_terminal_request: "supportsRunInTerminalRequest",
            /// Client supports memory references.
            supports_memory_references: "supportsMemoryReferences",
            /// Client supports progress reporting.
            supports_progress_reporting: "supportsProgressReporting",
            /// Client supports the invalidated event.
            supports_invalidated_event: "supportsInvalidatedEvent",
            /// Client supports the memory event.
            supports_memory_event: "supportsMemoryEvent",
        },
        String {
            /// The ID of the debug adapter.
            adapter_id: "adapterID",
        },
        Option<String> {
            /// The ID of the (frontend) client using this adapter.
            client_id: "clientID",
            /// The human readable name of the (frontend) client using this adapter.
            client_name: "clientName",
            /// The ISO-639 locale of the (frontend) client using this adapter, e.g. en-US
            /// or de-CH.
            locale: "locale",
        },
        Option<json::Value> {},
        Custom {},
        Option<Custom> {
            {
                type = PathFormat;
                closure = PathFormat::parse;
                /// Determines in what format paths are specified. The default is 'path', which
                /// is the native format.
                /// Values: 'path', 'uri', etc.
                path_format: "pathFormat";
            },
        },
    }
);
