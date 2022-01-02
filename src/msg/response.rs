use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct Response {
    request_seq: usize,
    response_type: ResponseType,
}

#[derive(Clone, Debug, Hash)]
pub enum ResponseType {}

impl Response {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<Response> {
        todo!()
    }
}
