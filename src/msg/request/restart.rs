use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct RestartRequest {}

impl RestartRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<RestartRequest> {
        todo!()
    }
}
