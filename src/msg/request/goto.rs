request!(
    /// The request sets the location where the debuggee will continue to run.
    /// This makes it possible to skip the execution of code or to executed code again.
    /// The code between the current location and the goto target is not executed but skipped.
    /// The debug adapter first sends the response and then a 'stopped' event with reason 'goto'.
    /// Clients should only call this request if the capability 'supportsGotoTargetsRequest' is true (because only then goto targets exist that can be passed as arguments).
    GotoRequest {
        /// The location where the debuggee will continue to run.
        target_id | "targetId": u64,
        /// Set the goto target for this thread.
        thread_id | "threadId": u64,
    }
);

response!(
    /// Response to 'goto' request. This is just an acknowledgement, so no body field is required.
    GotoResponse {}
);
