use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct ConfigurationDoneRequest {}

impl ConfigurationDoneRequest {
    pub(crate) fn parse(
        msg: json::Map<String, json::Value>,
    ) -> anyhow::Result<ConfigurationDoneRequest> {
        todo!()
    }
}
