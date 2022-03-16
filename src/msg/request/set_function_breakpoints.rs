use crate::msg::dap_type::FunctionBreakpoint;

request!(
    /// Replaces all existing function breakpoints with new function breakpoints.
    /// To clear all function breakpoints, specify an empty array.
    /// When a function breakpoint is hit, a 'stopped' event (with reason 'function breakpoint') is generated.
    /// Clients should only call this request if the capability 'supportsFunctionBreakpoints' is true.
    SetFunctionBreakpointsRequest {
        /// The function names of the breakpoints.
        breakpoints | "breakpoints": Vec<FunctionBreakpoint>,
    }
);
