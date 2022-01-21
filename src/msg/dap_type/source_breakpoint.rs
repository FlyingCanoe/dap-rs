use anyhow::Error;
use fallible_iterator::{convert, FallibleIterator};
use serde_json as json;

use crate::utils::{get_optional_str, get_optional_u64, get_u64};

#[derive(Debug, Clone)]
pub struct SourceBreakpoint {
    /// The source line of the breakpoint or logpoint.
    line: u64,

    /// An optional source column of the breakpoint.
    column: Option<u64>,

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

    /// If this attribute exists and is non-empty, the backend must not 'break'
    /// (stop)
    /// but log the message instead. Expressions within {} are interpolated.
    /// The attribute is only honored by a debug adapter if the capability
    /// 'supportsLogPoints' is true.
    log_message: Option<String>,
}

impl SourceBreakpoint {
    pub(crate) fn parse(input: Option<&json::Value>) -> anyhow::Result<SourceBreakpoint> {
        let input = input.ok_or(Error::msg("parsing error"))?;
        let line = get_u64(input, "line")?;
        let column = get_optional_u64(input, "column")?;
        let condition = get_optional_str(input, "condition")?.map(str::to_owned);
        let hit_condition = get_optional_str(input, "hitCondition")?.map(str::to_owned);
        let log_message = get_optional_str(input, "logMessage")?.map(str::to_owned);

        let breakpoint = SourceBreakpoint {
            line,
            column,
            condition,
            hit_condition,
            log_message,
        };
        Ok(breakpoint)
    }

    pub(crate) fn parse_optional_vec(
        input: Option<&json::Value>,
    ) -> anyhow::Result<Option<Vec<SourceBreakpoint>>> {
        if let Some(input) = input {
            let input = input.as_array().ok_or(Error::msg("parsing error"))?;
            let breakpoint_iter = input
                .iter()
                .map(|value| SourceBreakpoint::parse(Some(value)));
            let breakpoint_list: Vec<_> = convert(breakpoint_iter).collect()?;
            Ok(Some(breakpoint_list))
        } else {
            Ok(None)
        }
    }
}
