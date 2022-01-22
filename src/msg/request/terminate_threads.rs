use crate::utils::parse_optional_u64_vec;

request!(
    TerminateThreadsRequest {
        /// Ids of threads to be terminated.
        thread_ids | "threadIds": Option<Vec<u64>> = parse_optional_u64_vec,
    }
);
