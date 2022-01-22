use crate::utils::parse_u64;

request!(
    ExceptionInfoRequest {
        /// Thread for which exception information should be retrieved.
        thread_id | "threadId": u64 = parse_u64,
    }
);
