use serde_json as json;

request!(
    type Response = ();

    RestartRequest | "restart" {
        no_debug | "noDebug": Option<bool>,
        restart | "__restart": Option<json::Value>,
    }
);
