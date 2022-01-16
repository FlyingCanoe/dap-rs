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
    pub(crate) fn parse(input: &json::Value) -> anyhow::Result<EvaluateCtx> {
        let input = input.as_str().ok_or(Error::msg("parsing error"))?;
        let ctx = match input {
            "watch" => EvaluateCtx::Watch,
            "repl" => EvaluateCtx::Repl,
            "hover" => EvaluateCtx::Hover,
            "clipboard" => EvaluateCtx::Clipboard,
            _ => bail!("parsing error"),
        };
        Ok(ctx)
    }
}
