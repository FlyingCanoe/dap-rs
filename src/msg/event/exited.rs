event!(
    /// The event indicates that the debuggee has exited and returns its exit code.
    ExitedEvent | "exited" {
        /// The exit code returned from the debuggee.
        exit_code | "exitCode": u64,
    }
);
