use crate::msg::dap_type::source::Source;
use crate::msg::dap_type::goto_target::GotoTarget;

request!(
    GotoTargetsRequest | "gotoTargets" {
        /// The source location for which the goto targets are determined.
        source | "source": Source,
        /// The line location for which the goto targets are determined.
        line | "line": u64,
    }
);

response!(
    /// Response to 'gotoTargets' request.
    GotoTargetsResponse {
        /// The possible goto targets of the specified location.
        targets | "targets": Vec<GotoTarget>,
    }
);
