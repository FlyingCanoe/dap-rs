use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct LoadedSourcesRequest {}

impl LoadedSourcesRequest {
    pub(crate) fn parse(
        msg: json::Map<String, json::Value>,
    ) -> anyhow::Result<LoadedSourcesRequest> {
        todo!()
    }
}
