use crate::utils::{parse_optional_bool, parse_u64};

request!(
    ContinueRequest {
        /// Specifies the active thread. If the debug adapter supports single thread
        /// execution (see 'supportsSingleThreadExecutionRequests') and the optional
        /// argument 'singleThread' is true, only the thread with this ID is resumed.
        thread_id | "threadId": u64 = parse_u64,

        /// If this optional flag is true, execution is resumed only for the thread
        /// with given 'threadId'.
        single_thread | "singleThread": Option<bool> = parse_optional_bool,
    }
);
