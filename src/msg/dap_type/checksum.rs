use crate::utils::parse_string;

use super::ChecksumAlgorithm;

dap_type_struct!(
    Checksum {
        /// The algorithm used to calculate this checksum.
        algorithm | "algorithm": ChecksumAlgorithm = ChecksumAlgorithm::parse,

        /// Value of the checksum.
        checksum | "checksum": String = parse_string,
    }
);
