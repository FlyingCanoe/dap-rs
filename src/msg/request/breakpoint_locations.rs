use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct BreakpointLocationRequest {}

impl BreakpointLocationRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<BreakpointLocationRequest> {
        todo!()
    }
}
