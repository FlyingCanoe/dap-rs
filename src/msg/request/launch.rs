use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct LaunchRequest {}

impl LaunchRequest {
    pub(crate) fn parse(msg: json::Map<String, json::Value>) -> anyhow::Result<LaunchRequest> {
        todo!()
    }
}
