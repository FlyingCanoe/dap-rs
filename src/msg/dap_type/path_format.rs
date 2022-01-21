use anyhow::bail;
use anyhow::Error;
use serde_json as json;

#[derive(Debug, Clone, Copy)]
pub enum PathFormat {
    Path,
    Uri,
}

impl PathFormat {
    pub(crate) fn parse_optional(
        input: Option<&json::Value>,
    ) -> anyhow::Result<Option<PathFormat>> {
        if let Some(input) = input {
            let path_format = match input.as_str().ok_or(Error::msg("parsing error"))? {
                "path" => PathFormat::Path,
                "uri" => PathFormat::Uri,
                _ => bail!("parsing error"),
            };
            Ok(Some(path_format))
        } else {
            Ok(None)
        }
    }
}
