use crate::msg::dap_type::ValueFormat;

request2!(
    SetExpressionRequest {
        /// The l-value expression to assign to.
        expression | "expression": String,

        /// The value expression to assign to the l-value expression.
        value | "value": String,

        /// Evaluate the expressions in the scope of this stack frame. If not
        /// specified, the expressions are evaluated in the global scope.
        frame_id | "frameId": Option<u64>,

        /// Specifies how the resulting value should be formatted.
        format | "format": Option<ValueFormat>,
    }
);
