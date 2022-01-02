use serde_json as json;

#[derive(Clone, Debug, Hash)]
pub struct SetExceptionBreakpoints {}

impl SetExceptionBreakpoints {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<SetExceptionBreakpoints> {
        todo!()
    }
}
