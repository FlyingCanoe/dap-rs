use crate::msg::dap_type::stepping_granularity::SteppingGranularity;

request!(
    type Response = ();

    /// The request executes one step (in the given granularity) for the specified thread and allows all other threads to run freely by resuming them.
    /// If the debug adapter supports single thread execution (see capability 'supportsSingleThreadExecutionRequests') setting the 'singleThread' argument to true prevents other suspended threads from resuming.
    /// The debug adapter first sends the response and then a 'stopped' event (with reason 'step') after the step has completed.
    NextRequest | "next" {
        /// Specifies the thread for which to resume execution for one step (of the given granularity).
        thread_id | "threadId": u64,
        /// Optional granularity to step. If no granularity is specified, a granularity of 'statement' is assumed.
        granularity | "granularity": Option<SteppingGranularity>,
        /// If this optional flag is true, all other suspended threads are not resumed.
        single_thread | "singleThread": Option<bool>,
    }
);
