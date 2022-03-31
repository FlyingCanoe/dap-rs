use crate::msg::dap_type::stepping_granularity::SteppingGranularity;

request!(
    type Response = ();

    /// The request resumes the given thread to step into a function/method and allows all other threads to run freely by resuming them.
    /// If the debug adapter supports single thread execution (see capability 'supportsSingleThreadExecutionRequests') setting the 'singleThread' argument to true prevents other suspended threads from resuming.
    /// If the request cannot step into a target, 'stepIn' behaves like the 'next' request.
    /// The debug adapter first sends the response and then a 'stopped' event (with reason 'step') after the step has completed.
    /// If there are multiple function/method calls (or other targets) on the source line,
    /// the optional argument 'targetId' can be used to control into which target the 'stepIn' should occur.
    /// The list of possible targets for a given source line can be retrieved via the 'stepInTargets' request.
    StepInRequest | "stepIn" {
        /// Specifies the thread for which to resume execution for one step-into (of the given granularity).
        thread_id | "threadId": u64,
        /// Optional id of the target to step into.
        target_id | "targetId": Option<u64>,
        /// Optional granularity to step. If no granularity is specified, a granularity of 'statement' is assumed.
        granularity | "granularity": Option<SteppingGranularity>,
        /// If this optional flag is true, all other suspended threads are not resumed.
        single_thread | "singleThread": Option<bool>,
    }
);
