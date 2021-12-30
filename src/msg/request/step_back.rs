use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct StepBackRequest {}

impl StepBackRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<StepBackRequest> {
        todo!()
    }
}
