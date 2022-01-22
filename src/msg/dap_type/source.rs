use serde_json as json;

use super::{Checksum, PresentationHint};

dap_type_struct!(
    Source {
        /// The short name of the source. Every source returned from the debug adapter
        /// has a name.
        /// When sending a source to the debug adapter this name is optional.
        name | "name": Option<String>,

        /// The path of the source to be shown in the UI.
        /// It is only used to locate and load the content of the source if no
        /// sourceReference is specified (or its value is 0).
        path | "path": Option<String>,

        /// If sourceReference > 0 the contents of the source must be retrieved through
        /// the SourceRequest (even if a path is specified).
        /// A sourceReference is only valid for a session, so it must not be used to
        /// persist a source.
        /// The value should be less than or equal to 2147483647 (2^31-1).
        source_reference | "sourceReference": Option<u64>,

        /// An optional hint for how to present the source in the UI.
        /// A value of 'deemphasize' can be used to indicate that the source is not
        /// available or that it is skipped on stepping.
        /// Values: 'normal', 'emphasize', 'deemphasize', etc.
        presentation_hint | "presentationHint": Option<PresentationHint>,

        /// The (optional) origin of this source: possible values 'internal module',
        /// 'inlined content from source map', etc.
        origin | "origin": Option<String>,

        /// An optional list of sources that are related to this source. These may be
        /// the source that generated this source.
        sources | "sources": Option<Vec<Source>>,

        /// Optional data that a debug adapter might want to loop through the client.
        /// The client should leave the data intact and persist it across sessions. The
        /// client should not interpret the data.
        adapter_data | "adapterData": Option<json::Value>,

        /// The checksums associated with this file.
        checksums | "checksums": Option<Vec<Checksum>>,
    }
);
