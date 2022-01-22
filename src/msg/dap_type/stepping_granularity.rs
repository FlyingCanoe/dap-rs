use anyhow::{bail, Error};
use serde_json as json;

#[derive(Clone, Debug)]
pub enum SteppingGranularity {
    Statement,
    Line,
    Instruction,
}

impl SteppingGranularity {
    pub(crate) fn parse(input: Option<&json::Value>) -> anyhow::Result<SteppingGranularity> {
        let input = input
            .ok_or(Error::msg("parsing error"))?
            .as_str()
            .ok_or(Error::msg("parsing error"))?;
        let output = match input {
            "statement" => SteppingGranularity::Statement,
            "line" => SteppingGranularity::Line,
            "instruction" => SteppingGranularity::Instruction,

            _ => bail!("parsing error"),
        };
        Ok(output)
    }

    pub(crate) fn parse_option(
        input: Option<&json::Value>,
    ) -> anyhow::Result<Option<SteppingGranularity>> {
        if input.is_some() {
            let output = SteppingGranularity::parse(input)?;
            Ok(Some(output))
        } else {
            Ok(None)
        }
    }
}
