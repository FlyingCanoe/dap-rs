request!(
    /// The request returns the variable scopes for a given stackframe ID.
    ScopesRequest {
        /// Retrieve the scopes for this stackframe.
        frame_id | "frameId": u64,
    }
);
