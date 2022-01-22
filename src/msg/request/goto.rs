use crate::utils::parse_u64;

request!(
    GotoRequest {
        /// Set the goto target for this thread.
        thread_id | "threadId": u64 = parse_u64,

        /// The location where the debuggee will continue to run.
        target_id | "targetId": u64 = parse_u64,
    }
);
