use crate::msg::dap_type::FunctionBreakpoint;

request!(
    SetFunctionBreakpointsRequest {
        /// The function names of the breakpoints.
        breakpoints | "breakpoints": Vec<FunctionBreakpoint>,
    }
);
