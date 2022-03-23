use crate::msg::dap_type::breakpoint_location::BreakpointLocation;
use crate::msg::dap_type::Source;
use crate::utils::{Parse, ToValue};

use serde_json as json;

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
            let source = Source::parse(args.get("source"))?;
            let line = u64::parse(args.get("line"))?;
            let column = Option::<u64>::parse(args.get("column"))?;
            let end_line = Option::<u64>::parse(args.get("endLine"))?;
            let end_column = Option::<u64>::parse(args.get("endColumn"))?;

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

impl ToValue for BreakpointLocationsRequest {
    fn to_value(self) -> serde_json::Value {
        let mut msg = serde_json::Map::new();
        let mut arguments = serde_json::Map::new();

        msg.insert(
            "type".to_string(),
            json::Value::String("response".to_string()),
        );
        msg.insert("command".to_string(), "breakpointLocations".to_value());

        arguments.insert("source".to_string(), self.source.to_value());
        arguments.insert("line".to_string(), self.line.to_value());
        arguments.insert("column".to_string(), self.column.to_value());
        arguments.insert("endLine".to_string(), self.end_line.to_value());
        arguments.insert("endColumn".to_string(), self.end_column.to_value());

        msg.insert("arguments".to_string(), json::Value::Object(arguments));
        json::Value::Object(msg)
    }
}

response!(
    /// Response to 'breakpointLocations' request.
    /// Contains possible locations for source breakpoints.
    BreakpointLocationsResponse | "breakpointLocations" {
        /// Sorted set of possible breakpoint locations.
        breakpoints | "breakpoints": Vec<BreakpointLocation>,
    }
);
