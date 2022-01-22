use anyhow::Error;
use fallible_iterator::{convert, FallibleIterator};
use serde_json as json;

use crate::utils::{parse_optional_string, parse_string};

#[derive(Debug, Clone)]
pub struct FunctionBreakpoint {
    /// The name of the function.
    name: String,
    /// An optional expression for conditional breakpoints.
    /// It is only honored by a debug adapter if the capability
    /// 'supportsConditionalBreakpoints' is true.
    condition: Option<String>,
    /// An optional expression that controls how many hits of the breakpoint are
    /// ignored.
    /// The backend is expected to interpret the expression as needed.
    /// The attribute is only honored by a debug adapter if the capability
    /// 'supportsHitConditionalBreakpoints' is true.
    hit_condition: Option<String>,
}

impl FunctionBreakpoint {
    pub(crate) fn parse(input: Option<&json::Value>) -> anyhow::Result<FunctionBreakpoint> {
        let input = input.ok_or(Error::msg("parsing error"))?;
        let name = parse_string(input.get("name"))?;
        let condition = parse_optional_string(input.get("condition"))?;
        let hit_condition = parse_optional_string(input.get("hitCondition"))?;

        let output = FunctionBreakpoint {
            name,
            condition,
            hit_condition,
        };
        Ok(output)
    }

    pub(crate) fn parse_vec(
        input: Option<&json::Value>,
    ) -> anyhow::Result<Vec<FunctionBreakpoint>> {
        let input = input.ok_or(Error::msg("parsing error"))?;
        let iter = input
            .as_array()
            .ok_or(Error::msg("parsing error"))?
            .iter()
            .map(|value| FunctionBreakpoint::parse(Some(value)));
        let output: Vec<_> = convert(iter).collect()?;
        Ok(output)
    }

    pub(crate) fn parse_optional_vec(
        input: Option<&json::Value>,
    ) -> anyhow::Result<Option<Vec<FunctionBreakpoint>>> {
        if input.is_some() {
            let output = FunctionBreakpoint::parse_vec(input)?;
            Ok(Some(output))
        } else {
            Ok(None)
        }
    }
}
