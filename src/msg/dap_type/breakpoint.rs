use crate::msg::dap_type::source::Source;

dap_type_struct!(
    /// Information about a Breakpoint created in setBreakpoints, setFunctionBreakpoints, setInstructionBreakpoints, or setDataBreakpoints.
    Breakpoint {
        /// The source where the breakpoint is located.
        source | "source": Option<Source>,
        /// An optional start column of the actual range covered by the breakpoint.
        column | "column": Option<u64>,
        /// An optional identifier for the breakpoint. It is needed if breakpoint events are used to update or remove breakpoints.
        id | "id": Option<u64>,
        /// An optional memory reference to where the breakpoint is set.
        instruction_reference | "instructionReference": Option<String>,
        /// The start line of the actual range covered by the breakpoint.
        line | "line": Option<u64>,
        /// An optional end column of the actual range covered by the breakpoint.
        /// If no end line is given, then the end column is assumed to be in the start line.
        end_column | "endColumn": Option<u64>,
        /// An optional offset from the instruction reference.
        /// This can be negative.
        offset | "offset": Option<u64>,
        /// If true breakpoint could be set (but not necessarily at the desired location).
        verified | "verified": bool,
        /// An optional message about the state of the breakpoint.
        /// This is shown to the user and can be used to explain why a breakpoint could not be verified.
        message | "message": Option<String>,
        /// An optional end line of the actual range covered by the breakpoint.
        end_line | "endLine": Option<u64>,
    }
);

builder!(
    type BuildedType = Breakpoint;

    BreakpointBuilder {
        /// optional field
        source: Option Source,
        column: Option u64,
        id: Option u64,
        instruction_reference: Option String,
        line: Option u64,
        end_column: Option u64,
        offset: Option u64,
        message: Option String,
        end_line: Option u64,
        /// non optional field
        verified: bool,
    }
);
