request!(
    PauseRequest {
        optional_args = false;
        u64 {
            /// Pause execution for this thread.
            thread_id: "threadId",
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
