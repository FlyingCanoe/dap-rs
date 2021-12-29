use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub enum Request {}

impl Request {
    pub(crate) fn parse(msg: json::Map<String, json::Value>) -> anyhow::Result<Request> {
        todo!()
    }
}
