use crate::msg::dap_type::data_breakpoint::DataBreakpoint;
use crate::msg::dap_type::breakpoint::Breakpoint;

request!(
    /// Replaces all existing data breakpoints with new data breakpoints.
    /// To clear all data breakpoints, specify an empty array.
    /// When a data breakpoint is hit, a 'stopped' event (with reason 'data breakpoint') is generated.
    /// Clients should only call this request if the capability 'supportsDataBreakpoints' is true.
    SetDataBreakpointsRequest | "setDataBreakpoints" {
        /// The contents of this array replaces all existing data breakpoints. An empty array clears all data breakpoints.
        breakpoints | "breakpoints": Vec<DataBreakpoint>,
    }
);

response!(
    /// Response to 'setDataBreakpoints' request.
    /// Returned is information about each breakpoint created by this request.
    SetDataBreakpointsResponse | "setDataBreakpoints" {
        /// Information about the data breakpoints. The array elements correspond to the elements of the input argument 'breakpoints' array.
        breakpoints | "breakpoints": Vec<Breakpoint>,
    }
);
