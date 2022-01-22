use crate::msg::dap_type::ValueFormat;
use crate::utils::{parse_string, parse_u64};

request2!(
    SetVariableRequest {
        /// The reference of the variable container.
        variables_reference | "variablesReference": u64 = parse_u64,

        /// The name of the variable in the container.
        name | "name": String = parse_string,

        /// The value of the variable.
        value | "value": String = parse_string,

        /// Specifies details on how to format the response value.
        format | "format": Option<ValueFormat> = ValueFormat::parse_option,
    }
);
