use crate::utils::parse_u64;

request!(
    RestartFrameRequest {
        /// Restart this stackframe.
        frame_id | "frameId": u64 = parse_u64,
    }
);
