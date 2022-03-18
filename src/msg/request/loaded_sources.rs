use crate::msg::dap_type::source::Source;

request!(
    /// Retrieves the set of all sources currently loaded by the debugged process.
    /// Clients should only call this request if the capability 'supportsLoadedSourcesRequest' is true.
    LoadedSourcesRequest {}
);

response!(
    /// Response to 'loadedSources' request.
    LoadedSourcesResponse {
        /// Set of loaded sources.
        sources | "sources": Vec<Source>,
    }
);
