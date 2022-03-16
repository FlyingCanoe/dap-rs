dap_type_struct!(
    /// A GotoTarget describes a code location that can be used as a target in the 'goto' request.
    /// The possible goto targets can be determined via the 'gotoTargets' request.
    GotoTarget {
        /// Unique identifier for a goto target. This is used in the goto request.
        id | "id": u64,
        /// The name of the goto target (shown in the UI).
        label | "label": String,
        /// Optional memory reference for the instruction pointer value represented by this target.
        instruction_pointer_reference | "instructionPointerReference": Option<String>,
        /// The line of the goto target.
        line | "line": u64,
        /// An optional end column of the range covered by the goto target.
        end_column | "endColumn": Option<u64>,
        /// An optional end line of the range covered by the goto target.
        end_line | "endLine": Option<u64>,
        /// An optional column of the goto target.
        column | "column": Option<u64>,
    }
);
