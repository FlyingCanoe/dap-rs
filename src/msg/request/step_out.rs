use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct StepOutRequest {}

impl StepOutRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<StepOutRequest> {
        todo!()
    }
}
