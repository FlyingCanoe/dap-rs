request!(
    TerminateThreadsRequest {
        /// Ids of threads to be terminated.
        thread_ids | "threadIds": Option<Vec<u64>>,
    }
);
