use crate::msg::dap_type::stepping_granularity::SteppingGranularity;

request!(
    type Response = ();

    /// The request resumes the given thread to step out (return) from a function/method and allows all other threads to run freely by resuming them.
    /// If the debug adapter supports single thread execution (see capability 'supportsSingleThreadExecutionRequests') setting the 'singleThread' argument to true prevents other suspended threads from resuming.
    /// The debug adapter first sends the response and then a 'stopped' event (with reason 'step') after the step has completed.
    StepOutRequest | "stepOut" {
        /// Specifies the thread for which to resume execution for one step-out (of the given granularity).
        thread_id | "threadId": u64,
        /// Optional granularity to step. If no granularity is specified, a granularity of 'statement' is assumed.
        granularity | "granularity": Option<SteppingGranularity>,
        /// If this optional flag is true, all other suspended threads are not resumed.
        single_thread | "singleThread": Option<bool>,
    }
);
