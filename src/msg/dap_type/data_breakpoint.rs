use anyhow::Error;
use fallible_iterator::{convert, FallibleIterator};
use serde_json as json;

use crate::utils::{get_optional_str, get_str};

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
    pub(crate) fn parse(input: Option<&json::Value>) -> anyhow::Result<DataBreakpoint> {
        let input = input.ok_or(Error::msg("parsing error"))?;
        let data_id = get_str(input, "dataId")?.to_owned();
        let condition = get_optional_str(input, "condition")?.map(str::to_owned);
        let hit_condition = get_optional_str(input, "hitCondition")?.map(str::to_owned);

        let breakpoint = DataBreakpoint {
            data_id,
            condition,
            hit_condition,
        };
        Ok(breakpoint)
    }

    pub(crate) fn parse_vec(input: Option<&json::Value>) -> anyhow::Result<Vec<DataBreakpoint>> {
        let iter = input
            .ok_or(Error::msg("parsing error"))?
            .as_array()
            .ok_or(Error::msg("parsing error"))?
            .iter()
            .map(|value| DataBreakpoint::parse(Some(value)));
        let breakpoint_vec: Vec<_> = convert(iter).collect()?;
        Ok(breakpoint_vec)
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
