use crate::msg::dap_type::value_format::ValueFormat;
use crate::msg::dap_type::variable_presentation_hint::VariablePresentationHint;

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
    type Response = EvaluateResponse;

    /// Evaluates the given expression in the context of the top most stack frame.
    /// The expression has access to any variables and arguments that are in scope.
    EvaluateRequest | "evaluate" {
        /// Evaluate the expression in the scope of this stack frame. If not specified, the expression is evaluated in the global scope.
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

response!(
    /// Response to 'evaluate' request.
    EvaluateResponse | "evaluate" {
        /// The number of indexed child variables.
        /// The client can use this optional information to present the variables in a paged UI and fetch them in chunks.
        /// The value should be less than or equal to 2147483647 (2^31-1).
        indexed_variables | "indexedVariables": Option<u64>,
        /// The optional type of the evaluate result.
        /// This attribute should only be returned by a debug adapter if the client has passed the value true for the 'supportsVariableType' capability of the 'initialize' request.
        r#type | "type": Option<String>,
        /// Properties of a evaluate result that can be used to determine how to render the result in the UI.
        presentation_hint | "presentationHint": Option<VariablePresentationHint>,
        /// If variablesReference is > 0, the evaluate result is structured and its children can be retrieved by passing variablesReference to the VariablesRequest.
        /// The value should be less than or equal to 2147483647 (2^31-1).
        variables_reference | "variablesReference": u64,
        /// Optional memory reference to a location appropriate for this result.
        /// For pointer type eval results, this is generally a reference to the memory address contained in the pointer.
        /// This attribute should be returned by a debug adapter if the client has passed the value true for the 'supportsMemoryReferences' capability of the 'initialize' request.
        memory_reference | "memoryReference": Option<String>,
        /// The number of named child variables.
        /// The client can use this optional information to present the variables in a paged UI and fetch them in chunks.
        /// The value should be less than or equal to 2147483647 (2^31-1).
        named_variables | "namedVariables": Option<u64>,
        /// The result of the evaluate request.
        result | "result": String,
    }
);
