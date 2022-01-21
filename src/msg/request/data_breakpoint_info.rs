request!(
    DataBreakpointInfoRequest | "dataBreakpointInfo" {
        /// Reference to the Variable container if the data breakpoint is requested for
        /// a child of the container.
        variables_reference | "variablesReference": Option<u64>,

        /// The name of the Variable's child to obtain data breakpoint information for.
        /// If variablesReference isn't provided, this can be an expression.
        name | "name": String,
    }
);
