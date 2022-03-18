request!(
    RestartFrameRequest | "restartFrame" {
        /// Restart this stackframe.
        frame_id | "frameId": u64,
    }
);

response!(
    /// Response to 'restartFrame' request. This is just an acknowledgement, so no body field is required.
    RestartFrameResponse {}
);
