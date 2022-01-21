use anyhow::Error;
use serde_json as json;

use super::ChecksumAlgorithm;
use crate::utils::get_str;

#[derive(Clone, Debug)]
pub struct Checksum {
    /// The algorithm used to calculate this checksum.
    algorithm: ChecksumAlgorithm,

    /// Value of the checksum.
    checksum: String,
}

impl Checksum {
    pub(crate) fn parse(input: Option<&json::Value>) -> anyhow::Result<Checksum> {
        let input = input.ok_or(Error::msg("parsing error"))?;
        let algorithm = get_str(input, "algorithm")?;
        let algorithm = ChecksumAlgorithm::parse(algorithm)?;
        let checksum = get_str(input, "checksum")?.to_owned();

        let checksum = Checksum {
            algorithm,
            checksum,
        };
        Ok(checksum)
    }
}
