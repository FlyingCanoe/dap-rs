use crate::msg::dap_type::SteppingGranularity;

request2!(
    StepOutRequest {
        /// Specifies the thread for which to resume execution for one step-out (of the
        /// given granularity).
        thread_id | "threadId": u64,

        /// If this optional flag is true, all other suspended threads are not resumed.
        single_thread | "singleThread": Option<bool>,

        /// Optional granularity to step. If no granularity is specified, a granularity
        /// of 'statement' is assumed.
        granularity | "granularity": Option<SteppingGranularity>,
    }
);
