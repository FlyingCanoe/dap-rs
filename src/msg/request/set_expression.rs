use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct SetExpressionRequest {}

impl SetExpressionRequest {
    pub(crate) fn parse(
        msg: json::Map<String, json::Value>,
    ) -> anyhow::Result<SetExpressionRequest> {
        todo!()
    }
}
