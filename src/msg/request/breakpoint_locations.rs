use crate::msg::dap_type::Source;

request!(
    BreakpointLocationRequest {
        optional_args = true;
        u64 {
            /// Start line of range to search possible breakpoint locations in. If only the
            /// line is specified, the request returns all possible locations in that line.
            line: "line",
        },
        Option<u64> {
            /// Optional start column of range to search possible breakpoint locations in.
            /// If no start column is given, the first column in the start line is assumed.
            column: "column",

            /// Optional end line of range to search possible breakpoint locations in. If
            /// no end line is given, then the end line is assumed to be the start line.
            end_line: "endLine",

            /// Optional end column of range to search possible breakpoint locations in. If
            /// no end column is given, then it is assumed to be in the last column of the
            /// end line.
            end_column: "endColumn",
        },
        Option<bool> {},
        String {},
        Custom {
            {
                type = Source;
                closure = Source::parse;
                /// The source location of the breakpoints; either 'source.path' or
                /// 'source.reference' must be specified.
                source: "source";
            },
        },
    }
);
