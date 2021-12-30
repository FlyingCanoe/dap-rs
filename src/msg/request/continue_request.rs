use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct ContinueRequest {}

impl ContinueRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<ContinueRequest> {
        todo!()
    }
}
