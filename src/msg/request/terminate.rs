request!(
    TerminateRequest {
        /// A value of true indicates that this 'terminate' request is part of a
        /// restart sequence.
        restart | "restart": Option<bool>,
    }
);
