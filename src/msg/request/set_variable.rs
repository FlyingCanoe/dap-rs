use crate::msg::dap_type::ValueFormat;

request!(
    SetVariableRequest {
        /// The reference of the variable container.
        variables_reference | "variablesReference": u64,

        /// The name of the variable in the container.
        name | "name": String,

        /// The value of the variable.
        value | "value": String,

        /// Specifies details on how to format the response value.
        format | "format": Option<ValueFormat>,
    }
);
