use crate::msg::dap_type::SteppingGranularity;

request!(
    NextRequest {
        optional_args = false;
        u64 {
            /// Specifies the thread for which to resume execution for one step (of the
            /// given granularity).
            thread_id: "threadId",
        },
        Option<u64> {},
        Option<Vec<u64>> {},
        Option<bool> {
            /// If this optional flag is true, all other suspended threads are not resumed.
            single_thread: "singleThread",
        },
        String {},
        Option<String> {},
        Option<json::Value> {},
        Custom {},
        Option<Custom> {
            {
                type = SteppingGranularity;
                closure = SteppingGranularity::parse;
                /// Optional granularity to step. If no granularity is specified, a granularity
                /// of 'statement' is assumed.
                granularity: "granularity";
            },
        },
    }
);
