use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct RestartFrameRequest {}

impl RestartFrameRequest {
    pub(crate) fn parse(
        msg: json::Map<String, json::Value>,
    ) -> anyhow::Result<RestartFrameRequest> {
        todo!()
    }
}
