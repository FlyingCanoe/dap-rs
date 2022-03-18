event!(
    /// The event signals that the progress reporting needs to updated with a new message and/or percentage.
    /// The client does not have to update the UI immediately, but the clients needs to keep track of the message and/or percentage values.
    /// This event should only be sent if the client has passed the value true for the 'supportsProgressReporting' capability of the 'initialize' request.
    ProgressUpdateEvent | "progressUpdate" {
        /// Optional, more detailed progress message. If omitted, the previous message (if any) is used.
        message | "message": Option<String>,
        /// Optional progress percentage to display (value range: 0 to 100). If omitted no percentage will be shown.
        percentage | "percentage": Option<f64>,
        /// The ID that was introduced in the initial 'progressStart' event.
        progress_id | "progressId": String,
    }
);
