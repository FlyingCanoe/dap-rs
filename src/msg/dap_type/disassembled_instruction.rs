use crate::msg::dap_type::source::Source;

dap_type_struct!(
    /// Represents a single disassembled instruction.
    DisassembledInstruction {
        /// Name of the symbol that corresponds with the location of this instruction, if any.
        symbol | "symbol": Option<String>,
        /// Text representing the instruction and its operands, in an implementation-defined format.
        instruction | "instruction": String,
        /// The column within the line that corresponds to this instruction, if any.
        column | "column": Option<u64>,
        /// The line within the source location that corresponds to this instruction, if any.
        line | "line": Option<u64>,
        /// Optional raw bytes representing the instruction and its operands, in an implementation-defined format.
        instruction_bytes | "instructionBytes": Option<String>,
        /// The address of the instruction. Treated as a hex value if prefixed with '0x', or as a decimal value otherwise.
        address | "address": String,
        /// Source location that corresponds to this instruction, if any.
        /// Should always be set (if available) on the first instruction returned,
        /// but can be omitted afterwards if this instruction maps to the same source file as the previous instruction.
        location | "location": Option<Source>,
        /// The end column of the range that corresponds to this instruction, if any.
        end_column | "endColumn": Option<u64>,
        /// The end line of the range that corresponds to this instruction, if any.
        end_line | "endLine": Option<u64>,
    }
);
