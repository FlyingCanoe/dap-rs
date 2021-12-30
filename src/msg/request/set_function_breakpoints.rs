use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct SetFunctionBreakpointRequest {}

impl SetFunctionBreakpointRequest {
    pub(crate) fn parse(
        msg: json::Map<String, json::Value>,
    ) -> anyhow::Result<SetFunctionBreakpointRequest> {
        todo!()
    }
}
