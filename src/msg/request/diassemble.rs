use crate::utils::{parse_optional_bool, parse_optional_u64, parse_string, parse_u64};

request2!(
    DiassambleRequest {
        /// Memory reference to the base location containing the instructions to
        /// disassemble.
        memory_reference | "memoryReference": String = parse_string,

        /// Optional offset (in bytes) to be applied to the reference location before
        /// disassembling. Can be negative.
        offset | "offset": Option<u64> = parse_optional_u64,

        /// Optional offset (in instructions) to be applied after the byte offset (if
        /// any) before disassembling. Can be negative.
        instruction_offset | "instructionOffset": Option<u64> = parse_optional_u64,

        /// Number of instructions to disassemble starting at the specified location
        /// and offset.
        /// An adapter must return exactly this u64 of instructions - any
        /// unavailable instructions should be replaced with an implementation-defined
        /// 'invalid instruction' value.
        instruction_count | "instructionCount": u64 = parse_u64,

        /// If true, the adapter should attempt to resolve memory addresses and other
        /// values to symbolic names.
        resolve_symbols | "resolveSymbols": Option<bool> = parse_optional_bool,
    }
);
