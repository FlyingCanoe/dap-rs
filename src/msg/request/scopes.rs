use crate::utils::parse_u64;

request2!(
    ScopesRequest {
        /// Retrieve the scopes for this stackframe.
        frame_id | "frameId": u64 = parse_u64,
    }
);
