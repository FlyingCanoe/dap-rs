use super::checksum_algorithm::ChecksumAlgorithm;

dap_type_struct!(
    /// The checksum of an item calculated by the specified algorithm.
    Checksum {
        /// The algorithm used to calculate this checksum.
        algorithm | "algorithm": ChecksumAlgorithm,
        /// Value of the checksum.
        checksum | "checksum": String,
    }
);
