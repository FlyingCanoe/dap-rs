use anyhow::Error;
use serde_json as json;

#[derive(Debug, Clone, Copy)]
pub struct ValueFormat {
    /// Display the value in hex.
    hex: Option<bool>,
}

impl ValueFormat {
    pub(crate) fn parse(input: Option<&json::Value>) -> anyhow::Result<ValueFormat> {
        let value_format = if let Some(hex) = input.ok_or(Error::msg("parsing error"))?.get("hex") {
            let hex = hex.as_bool().ok_or(Error::msg("parsing error"))?;
            ValueFormat { hex: Some(hex) }
        } else {
            ValueFormat { hex: None }
        };
        Ok(value_format)
    }

    pub(crate) fn parse_option(input: Option<&json::Value>) -> anyhow::Result<Option<ValueFormat>> {
        if input.is_some() {
            Ok(Some(ValueFormat::parse(input)?))
        } else {
            Ok(None)
        }
    }
}
