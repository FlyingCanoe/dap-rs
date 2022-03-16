use crate::msg::dap_type::ValueFormat;

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
