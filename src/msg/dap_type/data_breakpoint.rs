use serde_json as json;

use crate::utils::Parse;

#[derive(Debug, Clone)]
pub struct DataBreakpoint {
    /// An id representing the data. This id is returned from the
    /// dataBreakpointInfo request.
    data_id: String,
    /// An optional expression for conditional breakpoints.
    condition: Option<String>,
    /// An optional expression that controls how many hits of the breakpoint are
    /// ignored.
    /// The backend is expected to interpret the expression as needed.
    hit_condition: Option<String>,
}

impl Parse for DataBreakpoint {
    fn parse(input: Option<&json::Value>) -> anyhow::Result<DataBreakpoint> {
        todo!()
    }
}
