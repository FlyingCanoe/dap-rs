use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct LaunchRequest {}

impl LaunchRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<LaunchRequest> {
        todo!()
    }
}
