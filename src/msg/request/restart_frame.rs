request!(
    RestartFrameRequest | "restartFrame" {
        /// Restart this stackframe.
        frame_id | "frameId": u64,
    }
);
