request!(
    ReadMemoryRequest {
        optional_args = false;
        u64 {
            /// Number of bytes to read at the specified location and offset.
            count: "count",
        },
        Option<u64> {
            /// Optional offset (in bytes) to be applied to the reference location before
            /// reading data. Can be negative.
            offset: "offset",
        },
        Option<bool> {},
        String {
            /// Memory reference to the base location from which data should be read.
            memory_reference: "memoryReference",
        },
        Option<String> {},
        Option<json::Value> {},
        Custom {},
        Option<Custom> {},
    }
);
