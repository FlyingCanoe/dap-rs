request!(
    /// The request restarts execution of the specified stackframe.
    /// The debug adapter first sends the response and then a 'stopped' event (with reason 'restart') after the restart has completed.
    /// Clients should only call this request if the capability 'supportsRestartFrame' is true.
    RestartFrameRequest {
        /// Restart this stackframe.
        frame_id | "frameId": u64,
    }
);
