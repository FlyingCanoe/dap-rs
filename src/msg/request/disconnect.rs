use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct DisconnectRequest {}

impl DisconnectRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<DisconnectRequest> {
        todo!()
    }
}
