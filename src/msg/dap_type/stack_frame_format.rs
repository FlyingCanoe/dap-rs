use anyhow::Error;
use fallible_iterator::{convert, FallibleIterator};
use serde_json as json;

use crate::utils::parse_optional_bool;

#[derive(Debug, Clone)]
pub struct StackFrameFormat {
    /// Displays parameters for the stack frame.
    parameters: Option<bool>,
    /// Displays the types of parameters for the stack frame.
    parameter_types: Option<bool>,
    /// Displays the names of parameters for the stack frame.
    parameter_names: Option<bool>,
    /// Displays the values of parameters for the stack frame.
    parameter_values: Option<bool>,
    /// Displays the line u64 of the stack frame.
    line: Option<bool>,
    /// Displays the module of the stack frame.
    module: Option<bool>,
    /// Includes all stack frames, including those the debug adapter might
    /// otherwise hide.
    include_all: Option<bool>,
}

impl StackFrameFormat {
    pub(crate) fn parse(input: Option<&json::Value>) -> anyhow::Result<StackFrameFormat> {
        let input = input.ok_or(Error::msg("parsing error"))?;
        let parameters = parse_optional_bool(input.get("parameters"))?;
        let parameter_types = parse_optional_bool(input.get("parameterTypes"))?;
        let parameter_names = parse_optional_bool(input.get("parameterNames"))?;
        let parameter_values = parse_optional_bool(input.get("parameterValues"))?;
        let line = parse_optional_bool(input.get("line"))?;
        let module = parse_optional_bool(input.get("module"))?;
        let include_all = parse_optional_bool(input.get("includeAll"))?;

        let output = StackFrameFormat {
            parameters,
            parameter_types,
            parameter_names,
            parameter_values,
            line,
            module,
            include_all,
        };
        Ok(output)
    }

    pub(crate) fn parse_option(
        input: Option<&json::Value>,
    ) -> anyhow::Result<Option<StackFrameFormat>> {
        if input.is_some() {
            let output = StackFrameFormat::parse(input)?;
            Ok(Some(output))
        } else {
            Ok(None)
        }
    }

    pub(crate) fn parse_vec(input: Option<&json::Value>) -> anyhow::Result<Vec<StackFrameFormat>> {
        let input = input.ok_or(Error::msg("parsing error"))?;
        let iter = input
            .as_array()
            .ok_or(Error::msg("parsing error"))?
            .iter()
            .map(|value| StackFrameFormat::parse(Some(value)));
        let output: Vec<_> = convert(iter).collect()?;
        Ok(output)
    }

    pub(crate) fn parse_optional_vec(
        input: Option<&json::Value>,
    ) -> anyhow::Result<Option<Vec<StackFrameFormat>>> {
        if input.is_some() {
            let output = StackFrameFormat::parse_vec(input)?;
            Ok(Some(output))
        } else {
            Ok(None)
        }
    }
}
