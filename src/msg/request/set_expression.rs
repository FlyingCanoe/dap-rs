use crate::msg::dap_type::value_format::ValueFormat;
use crate::msg::dap_type::variable_presentation_hint::VariablePresentationHint;

request!(
    /// Evaluates the given 'value' expression and assigns it to the 'expression' which must be a modifiable l-value.
    /// The expressions have access to any variables and arguments that are in scope of the specified frame.
    /// Clients should only call this request if the capability 'supportsSetExpression' is true.
    /// If a debug adapter implements both setExpression and setVariable, a client will only use setExpression if the variable has an evaluateName property.
    SetExpressionRequest {
        /// Evaluate the expressions in the scope of this stack frame. If not specified, the expressions are evaluated in the global scope.
        frame_id | "frameId": Option<u64>,
        /// Specifies how the resulting value should be formatted.
        format | "format": Option<ValueFormat>,
        /// The value expression to assign to the l-value expression.
        value | "value": String,
        /// The l-value expression to assign to.
        expression | "expression": String,
    }
);

response!(
    /// Response to 'setExpression' request.
    SetExpressionResponse {
        /// The number of indexed child variables.
        /// The client can use this optional information to present the variables in a paged UI and fetch them in chunks.
        /// The value should be less than or equal to 2147483647 (2^31-1).
        indexed_variables | "indexedVariables": Option<u64>,
        /// The optional type of the value.
        /// This attribute should only be returned by a debug adapter if the client has passed the value true for the 'supportsVariableType' capability of the 'initialize' request.
        r#type | "type": Option<String>,
        /// The new value of the expression.
        value | "value": String,
        /// Properties of a value that can be used to determine how to render the result in the UI.
        presentation_hint | "presentationHint": Option<VariablePresentationHint>,
        /// If variablesReference is > 0, the value is structured and its children can be retrieved by passing variablesReference to the VariablesRequest.
        /// The value should be less than or equal to 2147483647 (2^31-1).
        variables_reference | "variablesReference": Option<u64>,
        /// The number of named child variables.
        /// The client can use this optional information to present the variables in a paged UI and fetch them in chunks.
        /// The value should be less than or equal to 2147483647 (2^31-1).
        named_variables | "namedVariables": Option<u64>,
    }
);
