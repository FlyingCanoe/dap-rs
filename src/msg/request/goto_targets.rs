use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct GotoTargetsRequest {}

impl GotoTargetsRequest {
    pub(crate) fn parse(msg: json::Map<String, json::Value>) -> anyhow::Result<GotoTargetsRequest> {
        todo!()
    }
}
