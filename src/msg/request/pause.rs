use crate::utils::parse_u64;

request2!(
    PauseRequest {
        /// Pause execution for this thread.
        thread_id | "threadId": u64 = parse_u64,
    }
);
