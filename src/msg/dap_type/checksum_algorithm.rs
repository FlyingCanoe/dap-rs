use anyhow::bail;

#[derive(Clone, Debug)]
pub enum ChecksumAlgorithm {
    MD5,
    SHA1,
    SHA256,
    Timestamp,
}

impl ChecksumAlgorithm {
    pub(crate) fn parse(input: &str) -> anyhow::Result<ChecksumAlgorithm> {
        let algorithm = match input {
            "timestamp" => ChecksumAlgorithm::Timestamp,
            "SHA256" => ChecksumAlgorithm::SHA256,
            "SHA1" => ChecksumAlgorithm::SHA1,
            "MD5" => ChecksumAlgorithm::MD5,
            _ => bail!("invalid field"),
        };
        Ok(algorithm)
    }
}
