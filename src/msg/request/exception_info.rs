request!(
    /// Retrieves the details of the exception that caused this event to be raised.
    /// Clients should only call this request if the capability 'supportsExceptionInfoRequest' is true.
    ExceptionInfoRequest {
        /// Thread for which exception information should be retrieved.
        thread_id | "threadId": u64,
    }
);
