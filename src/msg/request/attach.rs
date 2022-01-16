request!(
AttachRequest {
    optional_args = false;
    u64 {},
    Option<u64> {},
    Option<bool> {},
    String {},
    Option<json::Value> {
        /// Optional data from the previous, restarted session.
        /// The data is sent as the 'restart' attribute of the 'terminated' event.
        /// The client should leave the data intact.
        restart_info: "__restart",
    },
    Custom {},
}
);
