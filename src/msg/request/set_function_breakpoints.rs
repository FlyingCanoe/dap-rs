﻿use crate::msg::dap_type::breakpoint::Breakpoint;
use crate::msg::dap_type::function_breakpoint::FunctionBreakpoint;

request!(
    type Response = SetFunctionBreakpointsResponse;

    /// Replaces all existing function breakpoints with new function breakpoints.
    /// To clear all function breakpoints, specify an empty array.
    /// When a function breakpoint is hit, a 'stopped' event (with reason 'function breakpoint') is generated.
    /// Clients should only call this request if the capability 'supportsFunctionBreakpoints' is true.
    SetFunctionBreakpointsRequest | "setFunctionBreakpoints" {
        /// The function names of the breakpoints.
        breakpoints | "breakpoints": Vec<FunctionBreakpoint>,
    }
);

response!(
    /// Response to 'setFunctionBreakpoints' request.
    /// Returned is information about each breakpoint created by this request.
    SetFunctionBreakpointsResponse | "setFunctionBreakpoints" {
        /// Information about the breakpoints. The array elements correspond to the elements of the 'breakpoints' array.
        breakpoints | "breakpoints": Vec<Breakpoint>,
    }
);
