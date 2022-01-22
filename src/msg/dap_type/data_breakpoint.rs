use super::DataBreakpointAccessType;

dap_type_struct!(
    DataBreakpoint {
        /// An id representing the data. This id is returned from the
        /// dataBreakpointInfo request.
        data_id | "dataId": String,

        /// The access type of the data.
        access_type | "accessType": Option<DataBreakpointAccessType>,

        /// An optional expression for conditional breakpoints.
        condition | "condition": Option<String>,

        /// An optional expression that controls how many hits of the breakpoint are
        /// ignored.
        /// The backend is expected to interpret the expression as needed.
        hit_condition | "hitCondition": Option<String>,
    }
);
