request!(
    GotoRequest | "goto" {
        /// Set the goto target for this thread.
        thread_id | "threadId": u64 ,
    }
);

response!(
    /// Response to 'goto' request. This is just an acknowledgement, so no body field is required.
    GotoResponse | "goto" {}
);
