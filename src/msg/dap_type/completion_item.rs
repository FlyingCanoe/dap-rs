use crate::msg::dap_type::completion_item_type::CompletionItemType;

dap_type_struct!(
    /// CompletionItems are the suggestions returned from the CompletionsRequest.
    CompletionItem {
        /// If text is not falsy then it is inserted instead of the label.
        text | "text": Option<String>,
        /// This value determines the location (in the CompletionsRequest's 'text' attribute) where the completion text is added.
        /// If missing the text is added at the location specified by the CompletionsRequest's 'column' attribute.
        start | "start": Option<u64>,
        /// The item's type. Typically the client uses this information to render the item in the UI with an icon.
        r#type | "type": Option<CompletionItemType>,
        /// The label of this completion item. By default this is also the text that is inserted when selecting this completion.
        label | "label": String,
        /// Determines the length of the new selection after the text has been inserted (or replaced).
        /// The selection can not extend beyond the bounds of the completion text.
        /// If omitted the length is assumed to be 0.
        selection_length | "selectionLength": Option<u64>,
        /// Determines the start of the new selection after the text has been inserted (or replaced).
        /// The start position must in the range 0 and length of the completion text.
        /// If omitted the selection starts at the end of the completion text.
        selection_start | "selectionStart": Option<u64>,
        /// A string that should be used when comparing this item with other items. When `falsy` the label is used.
        sort_text | "sortText": Option<String>,
        /// A human-readable string with additional information about this item, like type or symbol information.
        detail | "detail": Option<String>,
        /// This value determines how many characters are overwritten by the completion text.
        /// If missing the value 0 is assumed which results in the completion text being inserted.
        length | "length": Option<u64>,
    }
);
