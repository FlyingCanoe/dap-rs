use crate::msg::dap_type::{Source, SourceBreakpoint};
request!(
    SetBreakpointsRequest {
        /// The source location of the breakpoints either 'source.path' or
        /// 'source.reference' must be specified.
        source | "source": Source,

        /// The code locations of the breakpoints.
        breakpoints | "breakpoints": Option<Vec<SourceBreakpoint>>,

        /// Deprecated: The code locations of the breakpoints.
        lines | "lines": Option<Vec<u64>>,

        /// A value of true indicates that the underlying source has been modified
        /// which results in new breakpoint locations.
        source_modified | "sourceModified": Option<bool>,
    }
);
