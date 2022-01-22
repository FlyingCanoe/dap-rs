use anyhow::Error;
use fallible_iterator::{convert, FallibleIterator};
use serde_json as json;

use crate::utils::{parse_optional_string, parse_string};

#[derive(Debug, Clone)]
pub struct ExceptionFilterOptions {
    /// ID of an exception filter returned by the 'exceptionBreakpointFilters'
    /// capability.
    filter_id: String,
    /// An optional expression for conditional exceptions.
    /// The exception will break into the debugger if the result of the condition
    /// is true.
    condition: Option<String>,
}

impl ExceptionFilterOptions {
    pub(crate) fn parse(input: Option<&json::Value>) -> anyhow::Result<ExceptionFilterOptions> {
        let input = input.ok_or(Error::msg("parsing error"))?;
        let filter_id = parse_string(input.get("filterId"))?;
        let condition = parse_optional_string(input.get("condition"))?;

        let output = ExceptionFilterOptions {
            filter_id,
            condition,
        };
        Ok(output)
    }

    pub(crate) fn parse_vec(
        input: Option<&json::Value>,
    ) -> anyhow::Result<Vec<ExceptionFilterOptions>> {
        let input = input.ok_or(Error::msg("parsing error"))?;
        let iter = input
            .as_array()
            .ok_or(Error::msg("parsing error"))?
            .iter()
            .map(|value| ExceptionFilterOptions::parse(Some(value)));
        let output: Vec<_> = convert(iter).collect()?;
        Ok(output)
    }

    pub(crate) fn parse_optional_vec(
        input: Option<&json::Value>,
    ) -> anyhow::Result<Option<Vec<ExceptionFilterOptions>>> {
        if input.is_some() {
            let output = ExceptionFilterOptions::parse_vec(input)?;
            Ok(Some(output))
        } else {
            Ok(None)
        }
    }
}
