use crate::msg::dap_type::scope::Scope;

request!(
    /// The request returns the variable scopes for a given stackframe ID.
    ScopesRequest | "scopes" {
        /// Retrieve the scopes for this stackframe.
        frame_id | "frameId": u64,
    }
);

response!(
    /// Response to 'scopes' request.
    ScopesResponse | "scopes" {
        /// The scopes of the stackframe. If the array has length zero, there are no scopes available.
        scopes | "scopes": Vec<Scope>,
    }
);
