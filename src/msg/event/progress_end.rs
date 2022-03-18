event!(
    /// The event signals the end of the progress reporting with an optional final message.
    /// This event should only be sent if the client has passed the value true for the 'supportsProgressReporting' capability of the 'initialize' request.
    ProgressEndEvent | "progressEnd" {
        /// The ID that was introduced in the initial 'ProgressStartEvent'.
        progress_id | "progressId": String,
        /// Optional, more detailed progress message. If omitted, the previous message (if any) is used.
        message | "message": Option<String>,
    }
);
