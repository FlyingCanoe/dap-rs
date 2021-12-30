use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct SetDataBreakpointRequest {}

impl SetDataBreakpointRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<SetDataBreakpointRequest> {
        todo!()
    }
}
