dap_type_enum!(
    /// The reason for the event.
    Reason {
        Other,
        Started | "started",
        Exited | "exited",
    }
);

event!(
    /// The event indicates that a thread has started or exited.
    ThreadEvent | "thread" {
        /// The identifier of the thread.
        thread_id | "threadId": u64,
        /// The reason for the event.
        reason | "reason": Reason,
    }
);
