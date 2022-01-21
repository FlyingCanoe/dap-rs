use serde_json as json;

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

impl DataBreakpoint {
    pub(crate) fn parse(input: &json::Value) -> anyhow::Result<DataBreakpoint> {
        todo!()
    }
}

// The access type of the data.
/*access_type: Option<DataBreakpointAccessType>,

let access_type = if let Some(value) = input.get("accessType") {
    let access_type = DataBreakpointAccessType::parse(input)?;
    Some(access_type)
} else {
    None
};*/
