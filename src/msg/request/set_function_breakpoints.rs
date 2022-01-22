use crate::msg::dap_type::FunctionBreakpoint;

request2!(
    SetFunctionBreakpointsRequest {
        /// The function names of the breakpoints.
        breakpoints | "breakpoints": Vec<FunctionBreakpoint> = FunctionBreakpoint::parse_vec,
    }
);
