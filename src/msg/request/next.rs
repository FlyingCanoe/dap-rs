use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct NextRequest {}

impl NextRequest {
    pub(crate) fn parse(msg: json::Map<String, json::Value>) -> anyhow::Result<NextRequest> {
        todo!()
    }
}
