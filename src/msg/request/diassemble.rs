use serde_json as json;

use crate::utils::{get_optional_bool, get_optional_u64, get_str, get_u64};

#[derive(Clone, Debug)]
pub struct DiassambleRequest {
    /// Memory reference to the base location containing the instructions to
    /// disassemble.
    memory_reference: String,

    /// Optional offset (in bytes) to be applied to the reference location before
    /// disassembling. Can be negative.
    offset: Option<u64>,

    /// Optional offset (in instructions) to be applied after the byte offset (if
    /// any) before disassembling. Can be negative.
    instruction_offset: Option<u64>,

    /// Number of instructions to disassemble starting at the specified location
    /// and offset.
    /// An adapter must return exactly this u64 of instructions - any
    /// unavailable instructions should be replaced with an implementation-defined
    /// 'invalid instruction' value.
    instruction_count: u64,

    /// If true, the adapter should attempt to resolve memory addresses and other
    /// values to symbolic names.
    resolve_symbols: Option<bool>,
}

impl DiassambleRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<DiassambleRequest> {
        let memory_reference = get_str(&msg, "memoryReference")?.to_owned();
        let offset = get_optional_u64(&msg, "offset")?;
        let instruction_offset = get_optional_u64(&msg, "instructionOffset")?;
        let instruction_count = get_u64(&msg, "instructionCount")?;
        let resolve_symbols = get_optional_bool(&msg, "resolveSymbols")?;

        let request = DiassambleRequest {
            memory_reference,
            offset,
            instruction_offset,
            instruction_count,
            resolve_symbols,
        };
        Ok(request)
    }
}
