use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct ExceptionInfoRequest {}

impl ExceptionInfoRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<ExceptionInfoRequest> {
        todo!()
    }
}
