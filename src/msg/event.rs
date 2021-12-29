use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub enum Event {}

impl Event {
    pub(crate) fn parse(msg: json::Map<String, json::Value>) -> anyhow::Result<Event> {
        todo!()
    }
}
