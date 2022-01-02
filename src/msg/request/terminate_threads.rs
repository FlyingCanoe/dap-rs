use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct TerminateThreadsRequest {}

impl TerminateThreadsRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<TerminateThreadsRequest> {
        todo!()
    }
}
