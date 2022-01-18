use anyhow::{bail, Error};
use serde_json as json;

#[derive(Clone, Copy, Debug)]
pub enum SteppingGranularity {
    Statement,
    Line,
    Instruction,
}

impl SteppingGranularity {
    pub(crate) fn parse(
        input: Option<&json::Value>,
    ) -> anyhow::Result<Option<SteppingGranularity>> {
        let input = input.ok_or(Error::msg("parsing error"))?;
        let input = input.as_str().ok_or(Error::msg("parsing error"))?;

        let granularity = match input {
            "statement" => SteppingGranularity::Statement,
            "line" => SteppingGranularity::Line,
            "instruction" => SteppingGranularity::Instruction,
            _ => bail!("parsing error"),
        };
        Ok(Some(granularity))
    }
}
