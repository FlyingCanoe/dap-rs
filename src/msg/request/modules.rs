use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct ModulesRequest {}

impl ModulesRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<ModulesRequest> {
        todo!()
    }
}
