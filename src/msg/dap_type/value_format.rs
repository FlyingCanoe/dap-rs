use anyhow::Error;
use serde_json as json;

#[derive(Debug, Clone, Copy)]
pub struct ValueFormat {
    /// Display the value in hex.
    hex: Option<bool>,
}

impl ValueFormat {
    pub(crate) fn parse(input: &json::Value) -> anyhow::Result<ValueFormat> {
        let value_format = if let Some(hex) = input.get("hex") {
            let hex = hex.as_bool().ok_or(Error::msg("parsing error"))?;
            ValueFormat { hex: Some(hex) }
        } else {
            ValueFormat { hex: None }
        };
        Ok(value_format)
    }
}
