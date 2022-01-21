use crate::utils::{parse_optional_u64, parse_string, parse_u64};

request2!(
    CompletionsRequest {
        /// Returns completions in the scope of this stack frame. If not specified, the
        /// completions are returned for the global scope.
        frame_id | "frameId": Option<u64> = parse_optional_u64,

        /// One or more source lines. Typically this is the text a user has typed into
        /// the debug console before he asked for completion.
        text | "text": String = parse_string,

        /// The character position for which to determine the completion proposals.
        column | "column": u64 = parse_u64,

        /// An optional line for which to determine the completion proposals. If
        /// missing the first line of the text is assumed.
        line | "line": Option<u64> = parse_optional_u64,
    }
);
