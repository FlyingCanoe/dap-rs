use serde_json as json;

use crate::utils::{get_optional_u64, get_str};

#[derive(Clone, Debug)]
pub struct DataBreakpointInfoRequest {
    /// Reference to the Variable container if the data breakpoint is requested for
    /// a child of the container.
    variables_reference: Option<u64>,

    /// The name of the Variable's child to obtain data breakpoint information for.
    /// If variablesReference isn't provided, this can be an expression.
    name: String,
}

impl DataBreakpointInfoRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<DataBreakpointInfoRequest> {
        let variables_reference = get_optional_u64(&msg, "variablesReference")?;
        let name = get_str(&msg, "name")?.to_owned();

        let request = DataBreakpointInfoRequest {
            variables_reference,
            name,
        };
        Ok(request)
    }
}
