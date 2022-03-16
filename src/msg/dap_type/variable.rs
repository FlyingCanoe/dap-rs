use crate::msg::dap_type::variable_presentation_hint::VariablePresentationHint;

dap_type_struct!(
    /// A Variable is a name/value pair.
    /// Optionally a variable can have a 'type' that is shown if space permits or when hovering over the variable's name.
    /// An optional 'kind' is used to render additional properties of the variable, e.g. different icons can be used to indicate that a variable is public or private.
    /// If the value is structured (has children), a handle is provided to retrieve the children with the VariablesRequest.
    /// If the number of named or indexed children is large, the numbers should be returned via the optional 'namedVariables' and 'indexedVariables' attributes.
    /// The client can use this optional information to present the children in a paged UI and fetch them in chunks.
    Variable {
        /// The variable's value.
        /// This can be a multi-line text, e.g. for a function the body of a function.
        /// For structured variables (which do not have a simple value), it is recommended to provide a one line representation of the structured object. This helps to identify the structured object in the collapsed state when its children are not yet visible.
        /// An empty string can be used if no value should be shown in the UI.
        value | "value": String,
        /// The number of indexed child variables.
        /// The client can use this optional information to present the children in a paged UI and fetch them in chunks.
        indexed_variables | "indexedVariables": Option<u64>,
        /// The type of the variable's value. Typically shown in the UI when hovering over the value.
        /// This attribute should only be returned by a debug adapter if the client has passed the value true for the 'supportsVariableType' capability of the 'initialize' request.
        r#type | "type": Option<String>,
        /// Optional evaluatable name of this variable which can be passed to the 'EvaluateRequest' to fetch the variable's value.
        evaluate_name | "evaluateName": Option<String>,
        /// Properties of a variable that can be used to determine how to render the variable in the UI.
        presentation_hint | "presentationHint": Option<VariablePresentationHint>,
        /// If variablesReference is > 0, the variable is structured and its children can be retrieved by passing variablesReference to the VariablesRequest.
        variables_reference | "variablesReference": u64,
        /// The variable's name.
        name | "name": String,
        /// Optional memory reference for the variable if the variable represents executable code, such as a function pointer.
        /// This attribute is only required if the client has passed the value true for the 'supportsMemoryReferences' capability of the 'initialize' request.
        memory_reference | "memoryReference": Option<String>,
        /// The number of named child variables.
        /// The client can use this optional information to present the children in a paged UI and fetch them in chunks.
        named_variables | "namedVariables": Option<u64>,
    }
);
