use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct DiassambleRequest {}

impl DiassambleRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<DiassambleRequest> {
        todo!()
    }
}
