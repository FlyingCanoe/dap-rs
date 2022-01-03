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

pub(crate) fn get_optional_str<'a>(
    input: &'a json::Value,
    key: &str,
) -> anyhow::Result<Option<&'a str>> {
    if let Some(value) = input.get(key) {
        let output = value.as_str().ok_or(Error::msg("parsing error"))?;
        Ok(Some(output))
    } else {
        Ok(None)
    }
}

pub(crate) fn get_optional_u64(input: &json::Value, key: &str) -> anyhow::Result<Option<u64>> {
    if let Some(value) = input.get(key) {
        let output = value.as_u64().ok_or(Error::msg("parsing error"))?;
        Ok(Some(output))
    } else {
        Ok(None)
    }
}

pub(crate) fn get_optional_list(
    input: &json::Value,
    key: &str,
) -> anyhow::Result<Option<Vec<json::Value>>> {
    if let Some(value) = input.get(key) {
        let output = value.as_array().ok_or(Error::msg("parsing error"))?;
        Ok(Some(output.clone()))
    } else {
        Ok(None)
    }
}
