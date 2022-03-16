﻿request!(
    /// Reads bytes from memory at the provided location.
    /// Clients should only call this request if the capability 'supportsReadMemoryRequest' is true.
    ReadMemoryRequest {
        /// Optional offset (in bytes) to be applied to the reference location before reading data. Can be negative.
        offset | "offset": Option<u64>,
        /// Memory reference to the base location from which data should be read.
        memory_reference | "memoryReference": String,
        /// Number of bytes to read at the specified location and offset.
        count | "count": u64,
    }
);
