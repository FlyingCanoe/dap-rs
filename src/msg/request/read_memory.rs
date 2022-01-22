use crate::utils::{parse_optional_u64, parse_string, parse_u64};

request!(
    ReadMemoryRequest {
        /// Memory reference to the base location from which data should be read.
        memory_reference | "memoryReference": String = parse_string,

        /// Optional offset (in bytes) to be applied to the reference location before
        /// reading data. Can be negative.
        offset | "offset": Option<u64> = parse_optional_u64,

        /// Number of bytes to read at the specified location and offset.
        count | "count": u64 = parse_u64,
    }
);
