use crate::msg::dap_type::Source;

request!(
    GotoTargetsRequest | "gotoTargets" {
        /// The source location for which the goto targets are determined.
        source | "source": Source,
        /// The line location for which the goto targets are determined.
        line | "line": u64,
    }
);
