use anyhow::Error;
use serde_json as json;

use crate::msg::dap_type::Source;
use crate::utils::{get_optional_u64, get_u64};

#[derive(Clone, Debug)]
pub struct BreakpointLocationRequest {
    /// The source location of the breakpoints; either 'source.path' or
    /// 'source.reference' must be specified.
    source: Option<Source>,

    /// Start line of range to search possible breakpoint locations in. If only the
    /// line is specified, the request returns all possible locations in that line.
    line: Option<u64>,

    /// Optional start column of range to search possible breakpoint locations in.
    /// If no start column is given, the first column in the start line is assumed.
    column: Option<u64>,

    /// Optional end line of range to search possible breakpoint locations in. If
    /// no end line is given, then the end line is assumed to be the start line.
    end_line: Option<u64>,

    /// Optional end column of range to search possible breakpoint locations in. If
    /// no end column is given, then it is assumed to be in the last column of the
    /// end line.
    end_column: Option<u64>,
}

impl BreakpointLocationRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<BreakpointLocationRequest> {
        let request;
        if let Some(args) = msg.get("arguments") {
            let line = get_u64(&msg, "line")?;
            let line = Some(line);

            let column = get_optional_u64(&msg, "column")?;
            let end_line = get_optional_u64(&msg, "endLine")?;
            let end_column = get_optional_u64(&msg, "endColumn")?;

            let source = msg.get("source").ok_or(Error::msg("source"))?;
            let source = Source::parse(source)?;
            let source = Some(source);

            request = BreakpointLocationRequest {
                source,
                line,
                column,
                end_line,
                end_column,
            }
        } else {
            request = BreakpointLocationRequest {
                source: None,
                line: None,
                column: None,
                end_line: None,
                end_column: None,
            };
        }
        Ok(request)
    }
}
