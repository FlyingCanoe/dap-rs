use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct WriteMemoryRequest {}

impl WriteMemoryRequest {
    pub(crate) fn parse(msg: json::Map<String, json::Value>) -> anyhow::Result<WriteMemoryRequest> {
        todo!()
    }
}
