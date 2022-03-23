request!(
    /// This optional request indicates that the client has finished initialization of the debug adapter.
    /// So it is the last request in the sequence of configuration requests (which was started by the 'initialized' event).
    /// Clients should only call this request if the capability 'supportsConfigurationDoneRequest' is true.
    ConfigurationDoneRequest | "configurationDone" {}
);

response!(
    /// Response to 'configurationDone' request. This is just an acknowledgement, so no body field is required.
    ConfigurationDoneResponse | "configurationDone" {}
);
