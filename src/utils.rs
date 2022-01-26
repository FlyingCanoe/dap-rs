use anyhow::Error;
use fallible_iterator::{convert, FallibleIterator};
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

pub(crate) fn get_optional_u64_vec(
    input: &json::Value,
    key: &str,
) -> anyhow::Result<Option<Vec<u64>>> {
    if let Some(value) = input.get(key) {
        let output = value
            .as_array()
            .ok_or(Error::msg("parsing error"))?
            .iter()
            .map(json::Value::as_u64)
            .map(|value| value.ok_or(Error::msg("parsing error")));

        let output = convert(output);
        let output: Vec<_> = output.collect()?;

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

pub(crate) fn get_optional_bool(input: &json::Value, key: &str) -> anyhow::Result<Option<bool>> {
    if let Some(value) = input.get(key) {
        let output = value.as_bool().ok_or(Error::msg("parsing error"))?;
        Ok(Some(output))
    } else {
        Ok(None)
    }
}

pub(crate) fn parse_bool(input: Option<&json::Value>) -> anyhow::Result<bool> {
    input
        .ok_or(Error::msg("parsing error"))?
        .as_bool()
        .ok_or(Error::msg("parsing error"))
}

pub(crate) fn parse_optional_bool(input: Option<&json::Value>) -> anyhow::Result<Option<bool>> {
    if let Some(value) = input {
        let output = value.as_bool().ok_or(Error::msg("parsing error"))?;
        Ok(Some(output))
    } else {
        Ok(None)
    }
}

pub(crate) fn parse_u64(input: Option<&json::Value>) -> anyhow::Result<u64> {
    input
        .ok_or(Error::msg("parsing error"))?
        .as_u64()
        .ok_or(Error::msg("parsing error"))
}

pub(crate) fn parse_optional_u64(input: Option<&json::Value>) -> anyhow::Result<Option<u64>> {
    if let Some(value) = input {
        let output = value.as_u64().ok_or(Error::msg("parsing error"))?;
        Ok(Some(output))
    } else {
        Ok(None)
    }
}

pub(crate) fn parse_optional_u64_vec(
    input: Option<&json::Value>,
) -> anyhow::Result<Option<Vec<u64>>> {
    if let Some(value) = input {
        let iter = value
            .as_array()
            .ok_or(Error::msg("parsing error"))?
            .into_iter()
            .map(|value| parse_u64(Some(value)));

        let output: Vec<_> = convert(iter).collect()?;
        Ok(Some(output))
    } else {
        Ok(None)
    }
}

pub(crate) fn parse_string(input: Option<&json::Value>) -> anyhow::Result<String> {
    let output = input
        .ok_or(Error::msg("parsing error"))?
        .as_str()
        .ok_or(Error::msg("parsing error"))?
        .to_owned();

    Ok(output)
}

pub(crate) fn parse_optional_string(input: Option<&json::Value>) -> anyhow::Result<Option<String>> {
    if let Some(value) = input {
        let output = value
            .as_str()
            .ok_or(Error::msg("parsing error"))?
            .to_owned();
        Ok(Some(output))
    } else {
        Ok(None)
    }
}

pub(crate) fn parse_string_vec(input: Option<&json::Value>) -> anyhow::Result<Vec<String>> {
    let iter = input
        .ok_or(Error::msg("parsing error"))?
        .as_array()
        .ok_or(Error::msg("parsing error"))?
        .into_iter()
        .map(|value| parse_string(Some(value)));

    let output: Vec<_> = convert(iter).collect()?;
    Ok(output)
}

pub(crate) fn parse_optional_string_vec(
    input: Option<&json::Value>,
) -> anyhow::Result<Option<Vec<String>>> {
    if let Some(value) = input {
        let iter = value
            .as_array()
            .ok_or(Error::msg("parsing error"))?
            .into_iter()
            .map(|value| parse_string(Some(value)));

        let output: Vec<_> = convert(iter).collect()?;
        Ok(Some(output))
    } else {
        Ok(None)
    }
}
