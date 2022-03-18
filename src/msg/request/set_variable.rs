use crate::msg::dap_type::value_format::ValueFormat;

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

response!(
    /// Response to 'setVariable' request.
    SetVariableResponse {
        /// The number of indexed child variables.
        /// The client can use this optional information to present the variables in a paged UI and fetch them in chunks.
        /// The value should be less than or equal to 2147483647 (2^31-1).
        indexed_variables | "indexedVariables": Option<u64>,
        /// If variablesReference is > 0, the new value is structured and its children can be retrieved by passing variablesReference to the VariablesRequest.
        /// The value should be less than or equal to 2147483647 (2^31-1).
        variables_reference | "variablesReference": Option<u64>,
        /// The type of the new value. Typically shown in the UI when hovering over the value.
        r#type | "type": Option<String>,
        /// The new value of the variable.
        value | "value": String,
        /// The number of named child variables.
        /// The client can use this optional information to present the variables in a paged UI and fetch them in chunks.
        /// The value should be less than or equal to 2147483647 (2^31-1).
        named_variables | "namedVariables": Option<u64>,
    }
);
