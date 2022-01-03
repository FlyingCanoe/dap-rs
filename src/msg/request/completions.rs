use anyhow::Error;
use serde_json as json;

use crate::utils::{get_optional_u64, get_str, get_u64};

#[derive(Clone, Debug)]
pub struct CompletionsRequest {
    /// Returns completions in the scope of this stack frame. If not specified, the
    /// completions are returned for the global scope.
    frame_id: Option<u64>,

    /// One or more source lines. Typically this is the text a user has typed into
    /// the debug console before he asked for completion.
    text: String,

    /// The character position for which to determine the completion proposals.
    column: u64,

    /// An optional line for which to determine the completion proposals. If
    /// missing the first line of the text is assumed.
    line: Option<u64>,
}

impl CompletionsRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<CompletionsRequest> {
        let args = msg.get("arguments").ok_or(Error::msg("invalid request"))?;

        let frame_id = get_optional_u64(&msg, "frameId")?;
        let text = get_str(&msg, "text")?.to_owned();
        let column = get_u64(&msg, "column")?;
        let line = get_optional_u64(&msg, "line")?;

        let request = CompletionsRequest {
            frame_id,
            text,
            column,
            line,
        };
        Ok(request)
    }
}
