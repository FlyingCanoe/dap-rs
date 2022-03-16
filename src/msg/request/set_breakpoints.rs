use crate::msg::dap_type::SourceBreakpoint;
use crate::msg::dap_type::Source;

request!(
    /// Sets multiple breakpoints for a single source and clears all previous breakpoints in that source.
    /// To clear all breakpoint for a source, specify an empty array.
    /// When a breakpoint is hit, a 'stopped' event (with reason 'breakpoint') is generated.
    SetBreakpointsRequest {
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
