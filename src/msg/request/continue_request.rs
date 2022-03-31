request!(
    type Response = ContinueResponse;

    /// The request resumes execution of all threads. If the debug adapter supports single thread execution (see capability 'supportsSingleThreadExecutionRequests') setting the 'singleThread' argument to true resumes only the specified thread. If not all threads were resumed, the 'allThreadsContinued' attribute of the response must be set to false.
    ContinueRequest | "continue" {
        /// Specifies the active thread. If the debug adapter supports single thread execution (see 'supportsSingleThreadExecutionRequests') and the optional argument 'singleThread' is true, only the thread with this ID is resumed.
        thread_id | "threadId": u64,
        /// If this optional flag is true, execution is resumed only for the thread with given 'threadId'.
        single_thread | "singleThread": Option<bool>,
    }
);

response!(
    /// Response to 'continue' request.
    ContinueResponse | "continue" {
        /// The value true (or a missing property) signals to the client that all threads have been resumed. The value false must be returned if not all threads were resumed.
        all_threads_continued | "allThreadsContinued": Option<bool>,
    }
);
