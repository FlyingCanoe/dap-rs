use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct TerminateRequest {}

impl TerminateRequest {
    pub(crate) fn parse(msg: json::Map<String, json::Value>) -> anyhow::Result<TerminateRequest> {
        todo!()
    }
}
