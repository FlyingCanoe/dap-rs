use serde_json as json;

request!(
    RestartRequest {
        no_debug | "noDebug": Option<bool>,
        restart | "__restart": Option<json::Value>,
    }
);
