use serde_json as json;

use crate::utils::Parse;

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

impl Parse for InstructionBreakpoint {
    fn parse(input: Option<&json::Value>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}
