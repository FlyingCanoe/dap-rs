request!(
    ReadMemoryRequest | "readMemory" {
        /// Memory reference to the base location from which data should be read.
        memory_reference | "memoryReference": String,

        /// Optional offset (in bytes) to be applied to the reference location before
        /// reading data. Can be negative.
        offset | "offset": Option<u64>,

        /// Number of bytes to read at the specified location and offset.
        count | "count": u64,
    }
);
