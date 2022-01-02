use anyhow::Error;
use serde_json as json;

pub(crate) fn get_str<'a>(input: &'a json::Value, key: &str) -> anyhow::Result<&'a str> {
    input
        .get(key)
        .map_or(None, json::Value::as_str)
        .ok_or(Error::msg("parsing error"))
}

pub(crate) fn get_u64(input: &json::Value, key: &str) -> anyhow::Result<u64> {
    input
        .get(key)
        .map_or(None, json::Value::as_u64)
        .ok_or(Error::msg("parsing error"))
}
