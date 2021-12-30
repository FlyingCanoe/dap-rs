use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct SourceRequest {}

impl SourceRequest {
    pub(crate) fn parse(msg: json::Map<String, json::Value>) -> anyhow::Result<SourceRequest> {
        todo!()
    }
}
