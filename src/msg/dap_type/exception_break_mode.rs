use anyhow::{bail, Error};
use serde_json as json;

#[derive(Clone, Debug)]
pub enum ExceptionBreakMode {
    Never,
    Always,
    Unhandled,
    UserUnhandled,
}

impl ExceptionBreakMode {
    pub(crate) fn parse(input: Option<&json::Value>) -> anyhow::Result<ExceptionBreakMode> {
        let input = input
            .ok_or(Error::msg("parsing error"))?
            .as_str()
            .ok_or(Error::msg("parsing error"))?;
        let output = match input {
            "never" => ExceptionBreakMode::Never,
            "always" => ExceptionBreakMode::Always,
            "unhandled" => ExceptionBreakMode::Unhandled,
            "userUnhandled" => ExceptionBreakMode::UserUnhandled,

            _ => bail!("parsing error"),
        };
        Ok(output)
    }
}
