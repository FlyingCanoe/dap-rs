use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct EvaluateRequest {}

impl EvaluateRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<EvaluateRequest> {
        todo!()
    }
}
