use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct StepInTargetRequest {}

impl StepInTargetRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<StepInTargetRequest> {
        todo!()
    }
}
