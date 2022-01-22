use anyhow::Error;
use fallible_iterator::{convert, FallibleIterator};
use serde_json as json;

use crate::utils::{parse_optional_string, parse_optional_u64, parse_string};

#[derive(Debug, Clone)]
pub struct InstructionBreakpoint {
    /// The instruction reference of the breakpoint.
    /// This should be a memory or instruction pointer reference from an
    /// EvaluateResponse, Variable, StackFrame, GotoTarget, or Breakpoint.
    instruction_reference: String,
    /// An optional offset from the instruction reference.
    /// This can be negative.
    offset: Option<u64>,
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

impl InstructionBreakpoint {
    pub(crate) fn parse(input: Option<&json::Value>) -> anyhow::Result<InstructionBreakpoint> {
        let input = input.ok_or(Error::msg("parsing error"))?;
        let instruction_reference = parse_string(input.get("instructionReference"))?;
        let offset = parse_optional_u64(input.get("offset"))?;
        let condition = parse_optional_string(input.get("condition"))?;
        let hit_condition = parse_optional_string(input.get("hitCondition"))?;

        let output = InstructionBreakpoint {
            instruction_reference,
            offset,
            condition,
            hit_condition,
        };
        Ok(output)
    }

    pub(crate) fn parse_vec(
        input: Option<&json::Value>,
    ) -> anyhow::Result<Vec<InstructionBreakpoint>> {
        let input = input.ok_or(Error::msg("parsing error"))?;
        let iter = input
            .as_array()
            .ok_or(Error::msg("parsing error"))?
            .iter()
            .map(|value| InstructionBreakpoint::parse(Some(value)));
        let output: Vec<_> = convert(iter).collect()?;
        Ok(output)
    }

    pub(crate) fn parse_optional_vec(
        input: Option<&json::Value>,
    ) -> anyhow::Result<Option<Vec<InstructionBreakpoint>>> {
        if input.is_some() {
            let output = InstructionBreakpoint::parse_vec(input)?;
            Ok(Some(output))
        } else {
            Ok(None)
        }
    }
}
