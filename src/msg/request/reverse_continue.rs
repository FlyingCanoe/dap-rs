use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct ReverseContinueRequest {}

impl ReverseContinueRequest {
    pub(crate) fn parse(
        msg: json::Map<String, json::Value>,
    ) -> anyhow::Result<ReverseContinueRequest> {
        todo!()
    }
}
