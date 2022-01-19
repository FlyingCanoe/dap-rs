request!(
    CompletionsRequest {
        optional_args = false;
        u64 {
            /// The character position for which to determine the completion proposals.
            column: "column",
        },
        Option<u64> {
            /// Returns completions in the scope of this stack frame. If not specified, the
            /// completions are returned for the global scope.
            frame_id: "frameId",

            /// An optional line for which to determine the completion proposals. If
            /// missing the first line of the text is assumed.
            line: "line",
        },
        Option<Vec<u64>> {},
        Option<bool> {},
        String {
            /// One or more source lines. Typically this is the text a user has typed into
            /// the debug console before he asked for completion.
            text: "text",
        },
        Option<String> {},
        Option<json::Value> {},
        Custom {},
        Option<Custom> {},
    }
);
