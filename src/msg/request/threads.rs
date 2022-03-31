use crate::msg::dap_type::thread::Thread;

request!(
    type Response = ThreadsResponse;

    /// The request retrieves a list of all threads.
    ThreadsRequest | "threads" {}
);

response!(
    /// Response to 'threads' request.
    ThreadsResponse | "threads" {
        /// All threads.
        threads | "threads": Vec<Thread>,
    }
);
