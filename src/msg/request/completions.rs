use crate::msg::dap_type::completion_item::CompletionItem;

request!(
    /// Returns a list of possible completions for a given caret position and text.
    /// Clients should only call this request if the capability 'supportsCompletionsRequest' is true.
    CompletionsRequest | "completions" {
        /// An optional line for which to determine the completion proposals. If missing the first line of the text is assumed.
        line | "line": Option<u64>,
        /// Returns completions in the scope of this stack frame. If not specified, the completions are returned for the global scope.
        frame_id | "frameId": Option<u64>,
        /// The character position for which to determine the completion proposals.
        column | "column": u64,
        /// One or more source lines. Typically this is the text a user has typed into the debug console before he asked for completion.
        text | "text": String,
    }
);

response!(
    /// Response to 'completions' request.
    CompletionsResponse | "completions" {
        /// The possible completions for .
        targets | "targets": Vec<CompletionItem>,
    }
);
