use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct CompletionsRequest {}

impl CompletionsRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<CompletionsRequest> {
        todo!()
    }
}
