use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct SetExpressionRequest {}

impl SetExpressionRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<SetExpressionRequest> {
        todo!()
    }
}
