use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct SetExceptionBreakpoints {}

impl SetExceptionBreakpoints {
    pub(crate) fn parse(
        msg: json::Map<String, json::Value>,
    ) -> anyhow::Result<SetExceptionBreakpoints> {
        todo!()
    }
}
