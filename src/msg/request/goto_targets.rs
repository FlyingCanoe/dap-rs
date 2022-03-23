use crate::msg::dap_type::goto_target::GotoTarget;
use crate::msg::dap_type::source::Source;

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
    GotoTargetsResponse | "gotoTargets" {
        /// The possible goto targets of the specified location.
        targets | "targets": Vec<GotoTarget>,
    }
);
