event!(
    /// The event signals that a long running operation is about to start and
    /// provides additional information for the client to set up a corresponding progress and cancellation UI.
    /// The client is free to delay the showing of the UI in order to reduce flicker.
    /// This event should only be sent if the client has passed the value true for the 'supportsProgressReporting' capability of the 'initialize' request.
    ProgressStartEvent | "progressStart" {
        /// Optional, more detailed progress message.
        message | "message": Option<String>,
        /// An ID that must be used in subsequent 'progressUpdate' and 'progressEnd' events to make them refer to the same progress reporting.
        /// IDs must be unique within a debug session.
        progress_id | "progressId": String,
        /// If true, the request that reports progress may be canceled with a 'cancel' request.
        /// So this property basically controls whether the client should use UX that supports cancellation.
        /// Clients that don't support cancellation are allowed to ignore the setting.
        cancellable | "cancellable": Option<bool>,
        /// Optional progress percentage to display (value range: 0 to 100). If omitted no percentage will be shown.
        percentage | "percentage": Option<f64>,
        /// Mandatory (short) title of the progress reporting. Shown in the UI to describe the long running operation.
        title | "title": String,
        /// The request ID that this progress report is related to. If specified a debug adapter is expected to emit
        /// progress events for the long running request until the request has been either completed or cancelled.
        /// If the request ID is omitted, the progress report is assumed to be related to some general activity of the debug adapter.
        request_id | "requestId": Option<u64>,
    }
);
