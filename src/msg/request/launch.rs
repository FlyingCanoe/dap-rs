request!(
    LaunchRequest {
        optional_args = false;
        u64 {},
        Option<u64> {},
        Option<bool> {
            /// If noDebug is true the launch request should launch the program without
            /// enabling debugging.
            no_debug: "noDebug",
        },
        String {},
        Option<String> {},
        Option<json::Value> {
            /// Optional data from the previous, restarted session.
            /// The data is sent as the 'restart' attribute of the 'terminated' event.
            /// The client should leave the data intact.
            restart_info: "__restart",
        },
        Custom {},
        Option<Custom> {},
    }
);
