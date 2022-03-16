use serde_json as json;

request!(
    /// The attach request is sent from the client to the debug adapter to attach to a debuggee that is already running.
    /// Since attaching is debugger/runtime specific, the arguments for this request are not part of this specification.
    AttachRequest {
        /// Optional data from the previous, restarted session.
        /// The data is sent as the 'restart' attribute of the 'terminated' event.
        /// The client should leave the data intact.
        restart | "__restart": Option<json::Value>,
    }
);
