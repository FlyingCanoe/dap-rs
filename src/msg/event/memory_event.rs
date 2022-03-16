event!(
    MemoryEvent {
        /// Memory reference of a memory range that has been updated.
        memory_reference_ | "memoryReference": String,

        /// Starting offset in bytes where memory has been updated. Can be negative.
        offset | "offset": u64,

        /// Number of bytes updated.
        count | "count": u64,
    }
);
