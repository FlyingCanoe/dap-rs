use serde_json as json;

request2!(
    AttachRequest {
        /// Optional data from the previous, restarted session.
        /// The data is sent as the 'restart' attribute of the 'terminated' event.
        /// The client should leave the data intact.
        restart | "__restart": Option<json::Value> = |value: Option<&json::Value>| -> anyhow::Result<Option<json::Value>> {
            Ok(value.cloned())
        },
    }
);
