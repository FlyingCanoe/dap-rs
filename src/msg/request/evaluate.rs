use crate::msg::dap_type::ValueFormat;

dap_type_enum!(
    /// The context in which the evaluate request is run.
    Context {
        Other,
        /// evaluate is run in a watch.
        Watch | "watch",
        /// evaluate is run from REPL console.
        Repl | "repl",
        /// evaluate is run from a data hover.
        Hover | "hover",
        /// evaluate is run to generate the value that will be stored in the clipboard.
        /// The attribute is only honored by a debug adapter if the capability 'supportsClipboardContext' is true.
        Clipboard | "clipboard",
    }
);

request!(
    EvaluateRequest | "evaluate" {
        /// The expression to evaluate.
        expression | "expression": String,

        /// Evaluate the expression in the scope of this stack frame. If not specified,
        /// the expression is evaluated in the global scope.
        frame_id | "frameId": Option<u64>,
        /// Specifies details on how to format the Evaluate result.
        /// The attribute is only honored by a debug adapter if the capability 'supportsValueFormattingOptions' is true.
        format | "format": Option<ValueFormat>,
        /// The expression to evaluate.
        expression | "expression": String,
        /// The context in which the evaluate request is run.
        context | "context": Option<Context>,
    }
);
