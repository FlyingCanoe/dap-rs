use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct ThreadsRequest {}

impl ThreadsRequest {
    pub(crate) fn parse(msg: json::Map<String, json::Value>) -> anyhow::Result<ThreadsRequest> {
        todo!()
    }
}
