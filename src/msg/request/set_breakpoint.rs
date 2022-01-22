use crate::msg::dap_type::{Source, SourceBreakpoint};
use crate::utils::{parse_optional_bool, parse_optional_u64_vec};

request!(
    SetBreakpointsRequest {
        /// The source location of the breakpoints either 'source.path' or
        /// 'source.reference' must be specified.
        source | "source": Source = Source::parse,

        /// The code locations of the breakpoints.
        breakpoints | "breakpoints": Option<Vec<SourceBreakpoint>> = SourceBreakpoint::parse_optional_vec,

        /// Deprecated: The code locations of the breakpoints.
        lines | "lines": Option<Vec<u64>> = parse_optional_u64_vec,

        /// A value of true indicates that the underlying source has been modified
        /// which results in new breakpoint locations.
        source_modified | "sourceModified": Option<bool> = parse_optional_bool,
    }
);
