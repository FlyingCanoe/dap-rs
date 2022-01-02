use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct SetVariableRequest {}

impl SetVariableRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<SetVariableRequest> {
        todo!()
    }
}
