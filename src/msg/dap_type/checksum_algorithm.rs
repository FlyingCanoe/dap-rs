use anyhow::{bail, Error};
use serde_json as json;

#[derive(Clone, Debug)]
pub enum ChecksumAlgorithm {
    MD5,
    SHA1,
    SHA256,
    Timestamp,
}

impl ChecksumAlgorithm {
    pub(crate) fn parse(input: Option<&json::Value>) -> anyhow::Result<ChecksumAlgorithm> {
        let input = input
            .ok_or(Error::msg("parsing error"))?
            .as_str()
            .ok_or(Error::msg("parsing error"))?;
        let output = match input {
            "MD5" => ChecksumAlgorithm::MD5,
            "SHA1" => ChecksumAlgorithm::SHA1,
            "SHA256" => ChecksumAlgorithm::SHA256,
            "timestamp" => ChecksumAlgorithm::Timestamp,

            _ => bail!("parsing error"),
        };
        Ok(output)
    }

    pub(crate) fn parse_option(
        input: Option<&json::Value>,
    ) -> anyhow::Result<Option<ChecksumAlgorithm>> {
        if input.is_some() {
            let output = ChecksumAlgorithm::parse(input)?;
            Ok(Some(output))
        } else {
            Ok(None)
        }
    }
}
