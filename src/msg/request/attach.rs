use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct AttachRequest {}

impl AttachRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<AttachRequest> {
        todo!()
    }
}
