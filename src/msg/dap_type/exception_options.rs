use serde_json as json;

use crate::utils::Parse;

use super::{ExceptionBreakMode, ExceptionPathSegment};

#[derive(Debug, Clone)]
pub struct ExceptionOptions {
    /// A path that selects a single or multiple exceptions in a tree. If 'path' is
    /// missing, the whole tree is selected.
    /// By convention the first segment of the path is a category that is used to
    /// group exceptions in the UI.
    path: Option<Vec<ExceptionPathSegment>>,
    /// Condition when a thrown exception should result in a break.
    break_mode: ExceptionBreakMode,
}

impl Parse for ExceptionOptions {
    fn parse(input: Option<&json::Value>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}
