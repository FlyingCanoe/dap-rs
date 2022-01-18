request!(
    RestartFrameRequest {
        optional_args = false;
        u64 {
            /// Restart this stackframe.
            frame_id: "frameId",
        },
        Option<u64> {},
        Option<bool> {},
        String {},
        Option<String> {},
        Option<json::Value> {},
        Custom {},
        Option<Custom> {},
    }
);
