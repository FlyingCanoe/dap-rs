use anyhow::{bail, Error};
use serde_json as json;

#[derive(Clone, Debug)]
pub enum PresentationHint {
    Normal,
    Emphasize,
    Deemphasize,
}

impl PresentationHint {
    pub(crate) fn parse(input: Option<&json::Value>) -> anyhow::Result<PresentationHint> {
        let input = input
            .ok_or(Error::msg("parsing error"))?
            .as_str()
            .ok_or(Error::msg("parsing error"))?;
        let output = match input {
            "normal" => PresentationHint::Normal,
            "emphasize" => PresentationHint::Emphasize,
            "deemphasize" => PresentationHint::Deemphasize,

            _ => bail!("parsing error"),
        };
        Ok(output)
    }

    pub(crate) fn parse_option(
        input: Option<&json::Value>,
    ) -> anyhow::Result<Option<PresentationHint>> {
        if input.is_some() {
            let output = PresentationHint::parse(input)?;
            Ok(Some(output))
        } else {
            Ok(None)
        }
    }
}
