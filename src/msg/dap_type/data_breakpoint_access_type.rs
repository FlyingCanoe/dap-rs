use anyhow::{bail, Error};
use serde_json as json;

#[derive(Clone, Debug)]
pub enum DataBreakpointAccessType {
    Read,
    Write,
    ReadWrite,
}

impl DataBreakpointAccessType {
    pub(crate) fn parse(input: Option<&json::Value>) -> anyhow::Result<DataBreakpointAccessType> {
        let input = input.ok_or(Error::msg("parsing error"))?;
        let input = input.as_str().ok_or(Error::msg("parsing error"))?;
        let access_type = match input {
            "read" => DataBreakpointAccessType::Read,
            "write" => DataBreakpointAccessType::Write,
            "readWrite" => DataBreakpointAccessType::ReadWrite,

            _ => bail!("parsing error"),
        };
        Ok(access_type)
    }

    pub(crate) fn parse_option(
        input: Option<&json::Value>,
    ) -> anyhow::Result<Option<DataBreakpointAccessType>> {
        todo!()
    }
}
