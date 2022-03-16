use crate::msg::dap_type::ValueFormat;

request!(
    /// Set the variable with the given name in the variable container to a new value. Clients should only call this request if the capability 'supportsSetVariable' is true.
    /// If a debug adapter implements both setVariable and setExpression, a client will only use setExpression if the variable has an evaluateName property.
    SetVariableRequest {
        /// Specifies details on how to format the response value.
        format | "format": Option<ValueFormat>,
        /// The name of the variable in the container.
        name | "name": String,
        /// The value of the variable.
        value | "value": String,
        /// The reference of the variable container.
        variables_reference | "variablesReference": u64,
    }
);
