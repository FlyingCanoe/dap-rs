use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct GotoRequest {}

impl GotoRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<GotoRequest> {
        todo!()
    }
}
