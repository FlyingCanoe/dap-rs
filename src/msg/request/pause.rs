request!(
    PauseRequest | "pause" {
        /// Pause execution for this thread.
        thread_id | "threadId": u64,
    }
);

response!(
    /// Response to 'pause' request. This is just an acknowledgement, so no body field is required.
    PauseResponse | "pause" {}
);
