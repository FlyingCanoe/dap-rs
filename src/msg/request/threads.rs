use crate::msg::dap_type::thread::Thread;

request!(
    /// The request retrieves a list of all threads.
    ThreadsRequest {}
);

response!(
    /// Response to 'threads' request.
    ThreadsResponse {
        /// All threads.
        threads | "threads": Vec<Thread>,
    }
);
