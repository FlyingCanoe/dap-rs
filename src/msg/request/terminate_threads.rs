﻿request!(
    type Response = ();

    /// The request terminates the threads with the given ids.
    /// Clients should only call this request if the capability 'supportsTerminateThreadsRequest' is true.
    TerminateThreadsRequest | "terminateThreads" {
        /// Ids of threads to be terminated.
        thread_ids | "threadIds": Option<Vec<u64>>,
    }
);
