request!(
    DiassambleRequest {
        optional_args = false;
        u64 {
            /// Number of instructions to disassemble starting at the specified location
            /// and offset.
            /// An adapter must return exactly this number of instructions - any
            /// unavailable instructions should be replaced with an implementation-defined
            /// 'invalid instruction' value.
            instruction_count: "instructionCount",
        },
        Option<u64> {
            /// Optional offset (in bytes) to be applied to the reference location before
            /// disassembling. Can be negative.
            offset: "offset",
            /// Optional offset (in instructions) to be applied after the byte offset (if
            /// any) before disassembling. Can be negative.
            instruction_offset: "instructionOffset",
        },
        Option<bool> {
            /// If true, the adapter should attempt to resolve memory addresses and other
            /// values to symbolic names.
            resolve_symbols: "resolveSymbols",
        },
        String {
            /// Memory reference to the base location containing the instructions to
            /// disassemble.
            memory_reference: "memoryReference",
        },
        Option<String> {},
        Option<json::Value> {},
        Custom {},
        Option<Custom> {},
    }
);
