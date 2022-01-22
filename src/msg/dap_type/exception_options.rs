use anyhow::Error;
use fallible_iterator::{convert, FallibleIterator};
use serde_json as json;

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

impl ExceptionOptions {
    pub(crate) fn parse(input: Option<&json::Value>) -> anyhow::Result<ExceptionOptions> {
        let input = input.ok_or(Error::msg("parsing error"))?;
        let path = ExceptionPathSegment::parse_optional_vec(input.get("path"))?;
        let break_mode = ExceptionBreakMode::parse(input.get("breakMode"))?;

        let output = ExceptionOptions { path, break_mode };
        Ok(output)
    }

    pub(crate) fn parse_vec(input: Option<&json::Value>) -> anyhow::Result<Vec<ExceptionOptions>> {
        let input = input.ok_or(Error::msg("parsing error"))?;
        let iter = input
            .as_array()
            .ok_or(Error::msg("parsing error"))?
            .iter()
            .map(|value| ExceptionOptions::parse(Some(value)));
        let output: Vec<_> = convert(iter).collect()?;
        Ok(output)
    }

    pub(crate) fn parse_optional_vec(
        input: Option<&json::Value>,
    ) -> anyhow::Result<Option<Vec<ExceptionOptions>>> {
        if input.is_some() {
            let output = ExceptionOptions::parse_vec(input)?;
            Ok(Some(output))
        } else {
            Ok(None)
        }
    }
}
