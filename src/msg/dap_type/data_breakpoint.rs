use super::DataBreakpointAccessType;

dap_type_struct!(
    /// Properties of a data breakpoint passed to the setDataBreakpoints request.
    DataBreakpoint {
        /// An id representing the data. This id is returned from the dataBreakpointInfo request.
        data_id | "dataId": String,
        /// An optional expression for conditional breakpoints.
        condition | "condition": Option<String>,
        /// An optional expression that controls how many hits of the breakpoint are ignored.
        /// The backend is expected to interpret the expression as needed.
        hit_condition | "hitCondition": Option<String>,
        /// The access type of the data.
        access_type | "accessType": Option<DataBreakpointAccessType>,
    }
);
