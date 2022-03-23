use serde_json as json;

request!(
    RestartRequest | "restart" {
        no_debug | "noDebug": Option<bool>,
        restart | "__restart": Option<json::Value>,
    }
);

response!(
    /// Response to 'restart' request. This is just an acknowledgement, so no body field is required.
    RestartResponse | "restart" {}
);
