use anyhow::{bail, Error};
use serde_json as json;

#[derive(Debug, Clone, Copy)]
pub enum EvaluateCtx {
    Watch,
    Repl,
    Hover,
    Clipboard,
}

impl EvaluateCtx {
    pub(crate) fn parse(input: Option<&json::Value>) -> anyhow::Result<EvaluateCtx> {
        let input = input
            .ok_or(Error::msg("parsing error"))?
            .as_str()
            .ok_or(Error::msg("parsing error"))?;
        let ctx = match input {
            "watch" => EvaluateCtx::Watch,
            "repl" => EvaluateCtx::Repl,
            "hover" => EvaluateCtx::Hover,
            "clipboard" => EvaluateCtx::Clipboard,
            _ => bail!("parsing error"),
        };
        Ok(ctx)
    }

    pub(crate) fn parse_option(input: Option<&json::Value>) -> anyhow::Result<Option<EvaluateCtx>> {
        if input.is_some() {
            Ok(Some(EvaluateCtx::parse(input)?))
        } else {
            Ok(None)
        }
    }
}
