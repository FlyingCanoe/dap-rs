use crate::msg::dap_type::SteppingGranularity;

request!(
    NextRequest | "next" {
        /// Specifies the thread for which to resume execution for one step (of the
        /// given granularity).
        thread_id | "threadId": u64,
        /// Optional granularity to step. If no granularity is specified, a granularity of 'statement' is assumed.
        granularity | "granularity": Option<SteppingGranularity>,
        /// If this optional flag is true, all other suspended threads are not resumed.
        single_thread | "singleThread": Option<bool>,
    }
);
