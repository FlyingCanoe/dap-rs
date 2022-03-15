use crate::msg::dap_type::PathFormat;

request!(
    InitializeRequest {
        /// The ID of the (frontend) client using this adapter.
        client_id | "clientID": Option<String>,

        /// The human readable name of the (frontend) client using this adapter.
        client_name | "clientName": Option<String>,

        /// The ID of the debug adapter.
        adapter_id | "adapterID": String,

        /// The ISO-639 locale of the (frontend) client using this adapter, e.g. en-US
        /// or de-CH.
        locale | "locale": Option<String>,

        /// If true all line u64s are 1-based (default).
        lines_start_at1 | "linesStartAt1": Option<bool>,

        /// If true all column u64s are 1-based (default).
        columns_start_at1 | "columnsStartAt1": Option<bool>,

        /// Determines in what format paths are specified. The default is 'path', which
        /// is the native format.
        /// Values: 'path', 'uri', etc.
        path_format | "pathFormat": Option<PathFormat>,

        /// Client supports the optional type attribute for variables.
        supports_variable_type | "supportsVariableType": Option<bool>,

        /// Client supports the paging of variables.
        supports_variable_paging | "supportsVariablePaging": Option<bool>,

        /// Client supports the runInTerminal request.
        supports_run_in_terminal_request | "supportsRunInTerminalRequest": Option<bool>,

        /// Client supports memory references.
        supports_memory_references | "supportsMemoryReferences": Option<bool>,

        /// Client supports progress reporting.
        supports_progress_reporting | "supportsProgressReporting": Option<bool>,

        /// Client supports the invalidated event.
        supports_invalidated_event | "supportsInvalidatedEvent": Option<bool>,

        /// Client supports the memory event.
        supports_memory_event | "supportsMemoryEvent": Option<bool>,
    }
);
