use crate::msg::dap_type::DataBreakpoint;

request!(
    SetDataBreakpointsRequest {
        /// The contents of this array replaces all existing data breakpoints. An empty
        /// array clears all data breakpoints.
        breakpoints | "breakpoints": Vec<DataBreakpoint>,
    }
);
