use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct StackTraceRequest {}

impl StackTraceRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<StackTraceRequest> {
        todo!()
    }
}
