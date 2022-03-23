use crate::msg::dap_type::source_breakpoint::SourceBreakpoint;
use crate::msg::dap_type::source::Source;
use crate::msg::dap_type::breakpoint::Breakpoint;

request!(
    /// Sets multiple breakpoints for a single source and clears all previous breakpoints in that source.
    /// To clear all breakpoint for a source, specify an empty array.
    /// When a breakpoint is hit, a 'stopped' event (with reason 'breakpoint') is generated.
    SetBreakpointsRequest | "setBreakpoints" {
        /// A value of true indicates that the underlying source has been modified which results in new breakpoint locations.
        source_modified | "sourceModified": Option<bool>,
        /// Deprecated: The code locations of the breakpoints.
        lines | "lines": Option<Vec<u64>>,
        /// The source location of the breakpoints; either 'source.path' or 'source.reference' must be specified.
        source | "source": Source,
        /// The code locations of the breakpoints.
        breakpoints | "breakpoints": Option<Vec<SourceBreakpoint>>,
    }
);

response!(
    /// Response to 'setBreakpoints' request.
    /// Returned is information about each breakpoint created by this request.
    /// This includes the actual code location and whether the breakpoint could be verified.
    /// The breakpoints returned are in the same order as the elements of the 'breakpoints'
    /// (or the deprecated 'lines') array in the arguments.
    SetBreakpointsResponse | "setBreakpoints" {
        /// Information about the breakpoints.
        /// The array elements are in the same order as the elements of the 'breakpoints' (or the deprecated 'lines') array in the arguments.
        breakpoints | "breakpoints": Vec<Breakpoint>,
    }
);
