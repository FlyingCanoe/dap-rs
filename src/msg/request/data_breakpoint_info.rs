request!(
    DataBreakpointInfoRequest {
        optional_args = false;
        u64 {},
        Option<u64> {
            /// Reference to the Variable container if the data breakpoint is requested for
            /// a child of the container.
            variables_reference: "variablesReference",
        },
        Option<Vec<u64>> {},
        Option<bool> {},
        String {
            /// The name of the Variable's child to obtain data breakpoint information for.
            /// If variablesReference isn't provided, this can be an expression.
            name: "name",
        },
        Option<String> {},
        Option<json::Value> {},
        Custom {},
        Option<Custom> {},
    }
);
