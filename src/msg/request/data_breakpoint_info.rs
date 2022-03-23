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

response!(
    /// Response to 'dataBreakpointInfo' request.
    DataBreakpointInfoResponse | "dataBreakpointInfo" {
        /// An identifier for the data on which a data breakpoint can be registered with the setDataBreakpoints request or null if no data breakpoint is available.
        data_id | "dataId": Either<u64, String>,
        /// UI string that describes on what data the breakpoint is set on or why a data breakpoint is not available.
        description | "description": String,
        /// Optional attribute listing the available access types for a potential data breakpoint. A UI frontend could surface this information.
        access_types | "accessTypes": Option<Vec<DataBreakpointAccessType>>,
        /// Optional attribute indicating that a potential data breakpoint could be persisted across sessions.
        can_persist | "canPersist": Option<bool>,
    }
);
