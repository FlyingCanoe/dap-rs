request!(
    ExceptionInfoRequest {
        optional_args = false;
        u64 {
            /// Thread for which exception information should be retrieved.
            thread_id: "threadId",
        },
        Option<u64> {},
        Option<Vec<u64>> {},
        Option<bool> {},
        String {},
        Option<String> {},
        Option<json::Value> {},
        Custom {},
        Option<Custom> {},
    }
);
