use crate::msg::dap_type::*;
use crate::utils::{parse_optional_u64, parse_u64};

request2!(
    GotoTargetsRequest {
        /// The source location for which the goto targets are determined.
        source | "source": Source = Source::parse,

        /// The line location for which the goto targets are determined.
        line | "line": u64 = parse_u64,

        /// An optional column location for which the goto targets are determined.
        column | "column": Option<u64> = parse_optional_u64,
    }
);
