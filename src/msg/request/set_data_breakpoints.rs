use crate::msg::dap_type::DataBreakpoint;

request!(
    /// Replaces all existing data breakpoints with new data breakpoints.
    /// To clear all data breakpoints, specify an empty array.
    /// When a data breakpoint is hit, a 'stopped' event (with reason 'data breakpoint') is generated.
    /// Clients should only call this request if the capability 'supportsDataBreakpoints' is true.
    SetDataBreakpointsRequest {
        /// The contents of this array replaces all existing data breakpoints. An empty array clears all data breakpoints.
        breakpoints | "breakpoints": Vec<DataBreakpoint>,
    }
);
