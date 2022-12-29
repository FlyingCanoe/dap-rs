dap_type_enum!(
    /// Names of checksum algorithms that may be supported by a debug adapter.
    ChecksumAlgorithm {
        MD5 | "MD5",
        SHA1 | "SHA1",
        SHA256 | "SHA256",
        Timestamp | "timestamp",
    }
);
