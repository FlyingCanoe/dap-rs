use anyhow::Error;
use fallible_iterator::{convert, FallibleIterator};
use serde_json as json;

use crate::msg::dap_type::{Checksum, PresentationHint};
use crate::utils::{parse_optional_string, parse_optional_u64};

#[derive(Debug, Clone)]
pub struct Source {
    /// The short name of the source. Every source returned from the debug adapter
    /// has a name.
    /// When sending a source to the debug adapter this name is optional.
    name: Option<String>,
    /// The path of the source to be shown in the UI.
    /// It is only used to locate and load the content of the source if no
    /// sourceReference is specified (or its value is 0).
    path: Option<String>,
    /// If sourceReference > 0 the contents of the source must be retrieved through
    /// the SourceRequest (even if a path is specified).
    /// A sourceReference is only valid for a session, so it must not be used to
    /// persist a source.
    /// The value should be less than or equal to 2147483647 (2^31-1).
    source_reference: Option<u64>,
    /// An optional hint for how to present the source in the UI.
    /// A value of 'deemphasize' can be used to indicate that the source is not
    /// available or that it is skipped on stepping.
    /// Values: 'normal', 'emphasize', 'deemphasize', etc.
    presentation_hint: Option<PresentationHint>,
    /// The (optional) origin of this source: possible values 'internal module',
    /// 'inlined content from source map', etc.
    origin: Option<String>,
    /// An optional list of sources that are related to this source. These may be
    /// the source that generated this source.
    sources: Option<Vec<Source>>,
    /// Optional data that a debug adapter might want to loop through the client.
    /// The client should leave the data intact and persist it across sessions. The
    /// client should not interpret the data.
    adapter_data: Option<json::Value>,
    /// The checksums associated with this file.
    checksums: Option<Vec<Checksum>>,
}

impl Source {
    pub(crate) fn parse(input: Option<&json::Value>) -> anyhow::Result<Source> {
        let input = input.ok_or(Error::msg("parsing error"))?;
        let name = parse_optional_string(input.get("name"))?;
        let path = parse_optional_string(input.get("path"))?;
        let source_reference = parse_optional_u64(input.get("sourceReference"))?;
        let presentation_hint = PresentationHint::parse_option(input.get("presentationHint"))?;
        let origin = parse_optional_string(input.get("origin"))?;
        let sources = Source::parse_optional_vec(input.get("sources"))?;
        let adapter_data = input.get("adapterData").cloned();
        let checksums = Checksum::parse_optional_vec(input.get("checksums"))?;

        let output = Source {
            name,
            path,
            source_reference,
            presentation_hint,
            origin,
            sources,
            adapter_data,
            checksums,
        };
        Ok(output)
    }

    pub(crate) fn parse_option(input: Option<&json::Value>) -> anyhow::Result<Option<Source>> {
        if input.is_some() {
            let output = Source::parse(input)?;
            Ok(Some(output))
        } else {
            Ok(None)
        }
    }

    pub(crate) fn parse_vec(input: Option<&json::Value>) -> anyhow::Result<Vec<Source>> {
        let input = input.ok_or(Error::msg("parsing error"))?;
        let iter = input
            .as_array()
            .ok_or(Error::msg("parsing error"))?
            .iter()
            .map(|value| Source::parse(Some(value)));
        let output: Vec<_> = convert(iter).collect()?;
        Ok(output)
    }

    pub(crate) fn parse_optional_vec(
        input: Option<&json::Value>,
    ) -> anyhow::Result<Option<Vec<Source>>> {
        if input.is_some() {
            let output = Source::parse_vec(input)?;
            Ok(Some(output))
        } else {
            Ok(None)
        }
    }
}
