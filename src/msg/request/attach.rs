use serde_json as json;

#[derive(Clone, Debug)]
pub struct AttachRequest {
    restart_info: Option<json::Value>,
}

impl AttachRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<AttachRequest> {
        let restart_info = msg
            // get the request arguments
            .get("arguments")
            .map_or(None, json::Value::as_object)
            // get the restart info
            .map_or(None, |args| args.get("restart"))
            .cloned();

        let request = AttachRequest { restart_info };
        Ok(request)
    }
}
