use crate::msg::dap_type::*;

request!(
    GotoTargetsRequest {
        /// The source location for which the goto targets are determined.
        source | "source": Source,

        /// The line location for which the goto targets are determined.
        line | "line": u64,

        /// An optional column location for which the goto targets are determined.
        column | "column": Option<u64>,
    }
);
