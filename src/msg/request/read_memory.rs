use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct ReadMemoryRequest {}

impl ReadMemoryRequest {
    pub(crate) fn parse(msg: json::Map<String, json::Value>) -> anyhow::Result<ReadMemoryRequest> {
        todo!()
    }
}
