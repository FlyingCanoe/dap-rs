request!(
    ExceptionInfoRequest | "exceptionInfo" {
        /// Thread for which exception information should be retrieved.
        thread_id | "threadId": u64,
    }
);
