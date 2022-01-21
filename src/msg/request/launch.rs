use serde_json as json;

use crate::utils::parse_optional_bool;

request2!(
    LaunchRequest {
        /// If noDebug is true the launch request should launch the program without
        /// enabling debugging.
        no_debug | "noDebug": Option<bool> = parse_optional_bool,

        /// Optional data from the previous, restarted session.
        /// The data is sent as the 'restart' attribute of the 'terminated' event.
        /// The client should leave the data intact.
        restart | "__restart": Option<json::Value> = |value: Option<&json::Value>| -> anyhow::Result<Option<json::Value>>  {Ok(value.cloned())},
    }
);
