use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct InitializeRequest {}

impl InitializeRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<InitializeRequest> {
        todo!()
    }
}
