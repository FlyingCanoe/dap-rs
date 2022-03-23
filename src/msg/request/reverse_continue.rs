request!(
    /// The request resumes backward execution of all threads. If the debug adapter supports single thread execution (see capability 'supportsSingleThreadExecutionRequests') setting the 'singleThread' argument to true resumes only the specified thread. If not all threads were resumed, the 'allThreadsContinued' attribute of the response must be set to false.
    /// Clients should only call this request if the capability 'supportsStepBack' is true.
    ReverseContinueRequest | "reverseContinue" {
        /// Specifies the active thread. If the debug adapter supports single thread execution (see 'supportsSingleThreadExecutionRequests') and the optional argument 'singleThread' is true, only the thread with this ID is resumed.
        thread_id | "threadId": u64,
        /// If this optional flag is true, backward execution is resumed only for the thread with given 'threadId'.
        single_thread | "singleThread": Option<bool>,
    }
);

response!(
    /// Response to 'reverseContinue' request. This is just an acknowledgement, so no body field is required.
    ReverseContinueResponse | "reverseContinue" {}
);
