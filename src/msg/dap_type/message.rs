use std::collections::HashMap;

dap_type_struct!(
    /// A structured message object. Used to return errors from requests.
    Message {
        /// An optional url where additional information about this message can be found.
        url | "url": Option<String>,
        /// Unique identifier for the message.
        id | "id": u64,
        /// A format string for the message. Embedded variables have the form '{name}'.
        /// If variable name starts with an underscore character, the variable does not contain user data (PII) and can be safely used for telemetry purposes.
        format | "format": String,
        /// An object used as a dictionary for looking up the variables in the format string.
        variables | "variables": Option<HashMap<String, String>,
        /// If true show user.
        show_user | "showUser": Option<bool>,
        /// An optional label that is presented to the user as the UI for opening the url.
        url_label | "urlLabel": Option<String>,
        /// If true send to telemetry.
        send_telemetry | "sendTelemetry": Option<bool>,
    }
);
