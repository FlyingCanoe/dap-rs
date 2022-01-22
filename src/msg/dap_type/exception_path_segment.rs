use anyhow::Error;
use fallible_iterator::{convert, FallibleIterator};
use serde_json as json;

use crate::utils::{parse_optional_bool, parse_string_vec};

#[derive(Debug, Clone)]
pub struct ExceptionPathSegment {
    /// If false or missing this segment matches the names provided, otherwise it
    /// matches anything except the names provided.
    negate: Option<bool>,
    /// Depending on the value of 'negate' the names that should match or not
    /// match.
    names: Vec<String>,
}

impl ExceptionPathSegment {
    pub(crate) fn parse(input: Option<&json::Value>) -> anyhow::Result<ExceptionPathSegment> {
        let input = input.ok_or(Error::msg("parsing error"))?;
        let negate = parse_optional_bool(input.get("negate"))?;
        let names = parse_string_vec(input.get("names"))?;

        let output = ExceptionPathSegment { negate, names };
        Ok(output)
    }

    pub(crate) fn parse_vec(
        input: Option<&json::Value>,
    ) -> anyhow::Result<Vec<ExceptionPathSegment>> {
        let input = input.ok_or(Error::msg("parsing error"))?;
        let iter = input
            .as_array()
            .ok_or(Error::msg("parsing error"))?
            .iter()
            .map(|value| ExceptionPathSegment::parse(Some(value)));
        let output: Vec<_> = convert(iter).collect()?;
        Ok(output)
    }

    pub(crate) fn parse_optional_vec(
        input: Option<&json::Value>,
    ) -> anyhow::Result<Option<Vec<ExceptionPathSegment>>> {
        if input.is_some() {
            let output = ExceptionPathSegment::parse_vec(input)?;
            Ok(Some(output))
        } else {
            Ok(None)
        }
    }
}
