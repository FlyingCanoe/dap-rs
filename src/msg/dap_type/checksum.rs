use super::ChecksumAlgorithm;

dap_type_struct!(
    Checksum {
        /// The algorithm used to calculate this checksum.
        algorithm | "algorithm": ChecksumAlgorithm,

        /// Value of the checksum.
        checksum | "checksum": String,
    }
);
