use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct SetDataBreakpointRequest {}

impl SetDataBreakpointRequest {
    pub(crate) fn parse(
        msg: json::Map<String, json::Value>,
    ) -> anyhow::Result<SetDataBreakpointRequest> {
        todo!()
    }
}
