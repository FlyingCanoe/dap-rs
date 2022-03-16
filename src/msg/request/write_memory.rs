request!(
    /// Writes bytes to memory at the provided location.
    /// Clients should only call this request if the capability 'supportsWriteMemoryRequest' is true.
    WriteMemoryRequest {
        /// Optional offset (in bytes) to be applied to the reference location before writing data. Can be negative.
        offset | "offset": Option<u64>,
        /// Memory reference to the base location to which data should be written.
        memory_reference | "memoryReference": String,
        /// Optional property to control partial writes. If true, the debug adapter should attempt to write memory even if the entire memory region is not writable. In such a case the debug adapter should stop after hitting the first byte of memory that cannot be written and return the number of bytes written in the response via the 'offset' and 'bytesWritten' properties.
        /// If false or missing, a debug adapter should attempt to verify the region is writable before writing, and fail the response if it is not.
        allow_partial | "allowPartial": Option<bool>,
        /// Bytes to write, encoded using base64.
        data | "data": String,
    }
);
