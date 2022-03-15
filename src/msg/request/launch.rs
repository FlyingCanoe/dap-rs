use serde_json as json;

request!(
    LaunchRequest {
        /// If noDebug is true the launch request should launch the program without
        /// enabling debugging.
        no_debug | "noDebug": Option<bool>,

        /// Optional data from the previous, restarted session.
        /// The data is sent as the 'restart' attribute of the 'terminated' event.
        /// The client should leave the data intact.
        restart | "__restart": Option<json::Value>,
    }
);
