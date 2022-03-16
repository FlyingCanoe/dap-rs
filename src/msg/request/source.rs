use crate::msg::dap_type::Source;

request!(
    /// The request retrieves the source code for a given source reference.
    SourceRequest {
        /// The reference to the source. This is the same as source.sourceReference.
        /// This is provided for backward compatibility since old backends do not understand the 'source' attribute.
        source_reference | "sourceReference": u64,
        /// Specifies the source content to load. Either source.path or source.sourceReference must be specified.
        source | "source": Option<Source>,
    }
);
