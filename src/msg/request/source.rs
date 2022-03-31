use crate::msg::dap_type::source::Source;

request!(
    type Response = SourceResponse;

    /// The request retrieves the source code for a given source reference.
    SourceRequest | "source" {
        /// The reference to the source. This is the same as source.sourceReference.
        /// This is provided for backward compatibility since old backends do not understand the 'source' attribute.
        source_reference | "sourceReference": u64,
        /// Specifies the source content to load. Either source.path or source.sourceReference must be specified.
        source | "source": Option<Source>,
    }
);

response!(
    /// Response to 'source' request.
    SourceResponse | "source" {
        /// Content of the source reference.
        content | "content": String,
        /// Optional content type (mime type) of the source.
        mime_type | "mimeType": Option<String>,
    }
);
