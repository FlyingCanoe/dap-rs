request!(
    /// Retrieves the set of all sources currently loaded by the debugged process.
    /// Clients should only call this request if the capability 'supportsLoadedSourcesRequest' is true.
    LoadedSourcesRequest {}
);
