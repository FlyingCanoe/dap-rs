use crate::msg::dap_type::breakpoint_location::BreakpointLocation;
use crate::msg::dap_type::Source;
use crate::utils::Parse;

#[derive(Clone, Debug)]
pub struct BreakpointLocationsRequest {
    /// The source location of the breakpoints either 'source.path' or
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

impl BreakpointLocationsRequest {
    pub(crate) fn parse(msg: serde_json::Value) -> anyhow::Result<BreakpointLocationsRequest> {
        if let Some(args) = msg.get("arguments") {
            let source = Source::parse(msg.get("source"))?;
            let line = u64::parse(msg.get("line"))?;
            let column = Option::<u64>::parse(msg.get("column"))?;
            let end_line = Option::<u64>::parse(msg.get("endLine"))?;
            let end_column = Option::<u64>::parse(msg.get("endColumn"))?;

            let request = BreakpointLocationsRequest {
                source: Some(source),
                line: Some(line),
                column,
                end_line,
                end_column,
            };
            Ok(request)
        } else {
            let request = BreakpointLocationsRequest {
                source: None,
                line: None,
                column: None,
                end_line: None,
                end_column: None,
            };
            Ok(request)
        }
    }
}

response!(
    /// Response to 'breakpointLocations' request.
    /// Contains possible locations for source breakpoints.
    BreakpointLocationsResponse {
        /// Sorted set of possible breakpoint locations.
        breakpoints | "breakpoints": Vec<BreakpointLocation>,
    }
);
