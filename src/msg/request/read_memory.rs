request!(
    /// Reads bytes from memory at the provided location.
    /// Clients should only call this request if the capability 'supportsReadMemoryRequest' is true.
    ReadMemoryRequest | "readMemory" {
        /// Optional offset (in bytes) to be applied to the reference location before reading data. Can be negative.
        offset | "offset": Option<u64>,
        /// Memory reference to the base location from which data should be read.
        memory_reference | "memoryReference": String,
        /// Number of bytes to read at the specified location and offset.
        count | "count": u64,
    }
);

response!(
    /// Response to 'readMemory' request.
    ReadMemoryResponse | "readMemory" {
        /// The bytes read from memory, encoded using base64.
        data | "data": Option<String>,
        /// The address of the first byte of data returned.
        /// Treated as a hex value if prefixed with '0x', or as a decimal value otherwise.
        address | "address": String,
        /// The number of unreadable bytes encountered after the last successfully read byte.
        /// This can be used to determine the number of bytes that must be skipped before a subsequent 'readMemory' request will succeed.
        unreadable_bytes | "unreadableBytes": Option<u64>,
    }
);
