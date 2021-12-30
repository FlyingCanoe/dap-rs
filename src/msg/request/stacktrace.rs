use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct StackTraceRequest {}

impl StackTraceRequest {
    pub(crate) fn parse(msg: json::Map<String, json::Value>) -> anyhow::Result<StackTraceRequest> {
        todo!()
    }
}
