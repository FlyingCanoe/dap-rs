request!(
    GotoRequest {
        /// Set the goto target for this thread.
        thread_id | "threadId": u64,

        /// The location where the debuggee will continue to run.
        target_id | "targetId": u64,
    }
);
