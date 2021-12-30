use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct ScopesRequest {}

impl ScopesRequest {
    pub(crate) fn parse(msg: json::Map<String, json::Value>) -> anyhow::Result<ScopesRequest> {
        todo!()
    }
}
