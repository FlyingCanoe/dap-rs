use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct DataBreakPointInfoRequest {}

impl DataBreakPointInfoRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<DataBreakPointInfoRequest> {
        todo!()
    }
}
