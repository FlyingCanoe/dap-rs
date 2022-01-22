use serde_json as json;

use crate::utils::parse_optional_bool;

request!(
    RestartRequest {
        no_debug | "noDebug": Option<bool> = parse_optional_bool,
        restart | "__restart": Option<json::Value> = |value: Option<&json::Value>| -> anyhow::Result<Option<json::Value>> {
            Ok(value.cloned())
        },
    }
);
