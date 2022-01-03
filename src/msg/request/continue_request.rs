use anyhow::Error;
use serde_json as json;

use crate::utils::{get_optional_bool, get_u64};

#[derive(Clone, Debug)]
pub struct ContinueRequest {
    /// Specifies the active thread. If the debug adapter supports single thread
    /// execution (see 'supportsSingleThreadExecutionRequests') and the optional
    /// argument 'singleThread' is true, only the thread with this ID is resumed.
    thread_id: u64,

    /// If this optional flag is true, execution is resumed only for the thread
    /// with given 'threadId'.
    single_thread: Option<bool>,
}

impl ContinueRequest {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<ContinueRequest> {
        let args = msg.get("arguments").ok_or(Error::msg("invalid request"))?;

        let thread_id = get_u64(&msg, "threadId")?;
        let single_thread = get_optional_bool(&msg, "singleThread")?;

        let request = ContinueRequest {
            thread_id,
            single_thread,
        };
        Ok(request)
    }
}
