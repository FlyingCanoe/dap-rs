use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct PauseRequest {}

impl PauseRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<PauseRequest> {
        todo!()
    }
}
