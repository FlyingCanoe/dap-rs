use crate::utils::parse_u64;

request2!(
    RestartFrameRequest {
        /// Restart this stackframe.
        frame_id | "frameId": u64 = parse_u64,
    }
);
