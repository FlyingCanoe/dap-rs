use crate::msg::dap_type::Source;

request!(
    GotoTargetsRequest {
        optional_args = false;
        u64 {
            /// The line location for which the goto targets are determined.
            line: "line",
        },
        Option<u64> {
            /// An optional column location for which the goto targets are determined.
            column: "column",
        },
        Option<bool> {},
        String {},
        Option<String> {},
        Option<json::Value> {},
        Custom {
            {
                type = Source;
                closure = Source::parse;
                source: "source";
            },
        },
        Option<Custom> {},
    }
);
