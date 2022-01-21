request!(
    ScopesRequest {
        optional_args = false;
        u64 {
            /// Retrieve the scopes for this stackframe.
            frame_id: "frameId",
        },
        Option<u64> {},
        Option<Vec<u64>> {},
        Option<bool> {},
        String {},
        Option<String> {},
        Option<json::Value> {},
        Custom {},
    }
);
