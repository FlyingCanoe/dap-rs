request!(
    PauseRequest | "pause" {
        /// Pause execution for this thread.
        thread_id | "threadId": u64,
    }
);
