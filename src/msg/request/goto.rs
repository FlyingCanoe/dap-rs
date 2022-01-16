request!(
    GotoRequest {
        optional_args = false;
        u64 {
            /// Set the goto target for this thread.
            thread_id: "threadId",
            /// The location where the debuggee will continue to run.
            target_id: "targetId",
        },
        Option<u64> {},
        Option<bool> {},
        String {},
        Option<String> {},
        Option<json::Value> {},
        Custom {},
        Option<Custom> {},
    }
);
