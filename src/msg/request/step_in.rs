use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct StepInRequest {}

impl StepInRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<StepInRequest> {
        todo!()
    }
}
