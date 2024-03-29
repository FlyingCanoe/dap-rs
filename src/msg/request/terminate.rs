﻿request!(
    type Response = ();

    /// The 'terminate' request is sent from the client to the debug adapter in order to give the debuggee a chance for terminating itself.
    /// Clients should only call this request if the capability 'supportsTerminateRequest' is true.
    TerminateRequest | "terminate" {
        /// A value of true indicates that this 'terminate' request is part of a restart sequence.
        restart | "restart": Option<bool>,
    }
);
