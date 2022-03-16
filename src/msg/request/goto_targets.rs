use crate::msg::dap_type::Source;

request!(
    /// This request retrieves the possible goto targets for the specified source location.
    /// These targets can be used in the 'goto' request.
    /// Clients should only call this request if the capability 'supportsGotoTargetsRequest' is true.
    GotoTargetsRequest {
        /// An optional column location for which the goto targets are determined.
        column | "column": Option<u64>,
        /// The source location for which the goto targets are determined.
        source | "source": Source,
        /// The line location for which the goto targets are determined.
        line | "line": u64,
    }
);
