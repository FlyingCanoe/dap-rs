use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct VariablesRequest {}

impl VariablesRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<VariablesRequest> {
        todo!()
    }
}
