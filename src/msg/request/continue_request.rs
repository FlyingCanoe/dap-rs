use serde_json as json;

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
        todo!()
    }
}
