use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct GotoRequest {}

impl GotoRequest {
    pub(crate) fn parse(msg: json::Map<String, json::Value>) -> anyhow::Result<GotoRequest> {
        todo!()
    }
}
