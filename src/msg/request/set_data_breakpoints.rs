use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct SetDataBreakpointsRequest {}

impl SetDataBreakpointsRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<SetDataBreakpointsRequest> {
        todo!()
    }
}
