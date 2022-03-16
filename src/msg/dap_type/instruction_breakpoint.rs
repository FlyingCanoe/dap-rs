dap_type_struct!(
    /// Properties of a breakpoint passed to the setInstructionBreakpoints request
    InstructionBreakpoint {
        /// An optional offset from the instruction reference.
        /// This can be negative.
        offset | "offset": Option<u64>,
        /// An optional expression for conditional breakpoints.
        /// It is only honored by a debug adapter if the capability 'supportsConditionalBreakpoints' is true.
        condition | "condition": Option<String>,
        /// An optional expression that controls how many hits of the breakpoint are ignored.
        /// The backend is expected to interpret the expression as needed.
        /// The attribute is only honored by a debug adapter if the capability 'supportsHitConditionalBreakpoints' is true.
        hit_condition | "hitCondition": Option<String>,
        /// The instruction reference of the breakpoint.
        /// This should be a memory or instruction pointer reference from an EvaluateResponse, Variable, StackFrame, GotoTarget, or Breakpoint.
        instruction_reference | "instructionReference": String,
    }
);
