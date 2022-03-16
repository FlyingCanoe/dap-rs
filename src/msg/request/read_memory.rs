request!(
    ReadMemoryRequest | "readMemory" {
        /// Memory reference to the base location from which data should be read.
        memory_reference | "memoryReference": String,
        /// Number of bytes to read at the specified location and offset.
        count | "count": u64,
    }
);
