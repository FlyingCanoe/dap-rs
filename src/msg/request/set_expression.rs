use crate::msg::dap_type::ValueFormat;
use crate::utils::{parse_optional_u64, parse_string};

request!(
    SetExpressionRequest {
        /// The l-value expression to assign to.
        expression | "expression": String = parse_string,

        /// The value expression to assign to the l-value expression.
        value | "value": String = parse_string,

        /// Evaluate the expressions in the scope of this stack frame. If not
        /// specified, the expressions are evaluated in the global scope.
        frame_id | "frameId": Option<u64> = parse_optional_u64,

        /// Specifies how the resulting value should be formatted.
        format | "format": Option<ValueFormat> = ValueFormat::parse_option,
    }
);
