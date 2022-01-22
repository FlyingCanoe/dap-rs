use anyhow::Error;
use fallible_iterator::{convert, FallibleIterator};
use serde_json as json;

use crate::utils::parse_string;

use super::ChecksumAlgorithm;

#[derive(Debug, Clone)]
pub struct Checksum {
    /// The algorithm used to calculate this checksum.
    algorithm: ChecksumAlgorithm,
    /// Value of the checksum.
    checksum: String,
}

impl Checksum {
    pub(crate) fn parse(input: Option<&json::Value>) -> anyhow::Result<Checksum> {
        let input = input.ok_or(Error::msg("parsing error"))?;
        let algorithm = ChecksumAlgorithm::parse(input.get("algorithm"))?;
        let checksum = parse_string(input.get("checksum"))?;

        let output = Checksum {
            algorithm,
            checksum,
        };
        Ok(output)
    }

    pub(crate) fn parse_vec(input: Option<&json::Value>) -> anyhow::Result<Vec<Checksum>> {
        let input = input.ok_or(Error::msg("parsing error"))?;
        let iter = input
            .as_array()
            .ok_or(Error::msg("parsing error"))?
            .iter()
            .map(|value| Checksum::parse(Some(value)));
        let output: Vec<_> = convert(iter).collect()?;
        Ok(output)
    }

    pub(crate) fn parse_optional_vec(
        input: Option<&json::Value>,
    ) -> anyhow::Result<Option<Vec<Checksum>>> {
        if input.is_some() {
            let output = Checksum::parse_vec(input)?;
            Ok(Some(output))
        } else {
            Ok(None)
        }
    }
}
