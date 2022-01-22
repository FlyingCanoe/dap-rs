use anyhow::{bail, Error};
use serde_json as json;

#[derive(Clone, Debug)]
pub enum VariablesFilter {
    Indexed,
    Named,
}

impl VariablesFilter {
    pub(crate) fn parse(input: Option<&json::Value>) -> anyhow::Result<VariablesFilter> {
        let input = input
            .ok_or(Error::msg("parsing error"))?
            .as_str()
            .ok_or(Error::msg("parsing error"))?;
        let output = match input {
            "indexed" => VariablesFilter::Indexed,
            "named" => VariablesFilter::Named,

            _ => bail!("parsing error"),
        };
        Ok(output)
    }

    pub(crate) fn parse_option(
        input: Option<&json::Value>,
    ) -> anyhow::Result<Option<VariablesFilter>> {
        if input.is_some() {
            let output = VariablesFilter::parse(input)?;
            Ok(Some(output))
        } else {
            Ok(None)
        }
    }
}
