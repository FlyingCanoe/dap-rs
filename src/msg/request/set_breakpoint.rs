use crate::msg::dap_type::{Source, SourceBreakpoint};

request!(
    SetBreakpointsRequest {
        optional_args = false;
        u64 {},
        Option<u64> {},
        Option<Vec<u64>> {
            /// Deprecated: The code locations of the breakpoints.
            lines: "lines",
        },
        Option<bool> {
            /// A value of true indicates that the underlying source has been modified
            /// which results in new breakpoint locations.
            source_modified: "sourceModified",
        },
        String {},
        Option<String> {},
        Option<json::Value> {},
        Custom {
            {
                type = Source;
                closure = Source::parse;
                source: "source";
            },
            {
                type = Option<Vec<SourceBreakpoint>>;
                closure = SourceBreakpoint::parse_optional_vec;
                breakpoints: "breakpoints";
            },
        },
    }
);
