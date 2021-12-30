use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct SetInstructionBreakpointsRequest {}

impl SetInstructionBreakpointsRequest {
    pub(crate) fn parse(
        msg: json::Map<String, json::Value>,
    ) -> anyhow::Result<SetInstructionBreakpointsRequest> {
        todo!()
    }
}
