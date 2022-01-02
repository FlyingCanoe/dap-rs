use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct SetBreakpointsRequest {}

impl SetBreakpointsRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<SetBreakpointsRequest> {
        todo!()
    }
}
