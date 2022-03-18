use crate::msg::dap_type::value_format::ValueFormat;
use crate::msg::dap_type::variable::Variable;

dap_type_enum!(
    /// Optional filter to limit the child variables to either named or indexed. If omitted, both types are fetched.
    Filter {
        Indexed | "indexed",
        Named | "named",
    }
);

request!(
    /// Retrieves all child variables for the given variable reference.
    /// An optional filter can be used to limit the fetched children to either named or indexed children.
    VariablesRequest {
        /// Optional filter to limit the child variables to either named or indexed. If omitted, both types are fetched.
        filter | "filter": Option<Filter>,
        /// The index of the first variable to return; if omitted children start at 0.
        start | "start": Option<u64>,
        /// Specifies details on how to format the Variable values.
        /// The attribute is only honored by a debug adapter if the capability 'supportsValueFormattingOptions' is true.
        format | "format": Option<ValueFormat>,
        /// The number of variables to return. If count is missing or 0, all variables are returned.
        count | "count": Option<u64>,
        /// The Variable reference.
        variables_reference | "variablesReference": u64,
    }
);

response!(
    /// Response to 'variables' request.
    VariablesResponse {
        /// All (or a range) of variables for the given variable reference.
        variables | "variables": Vec<Variable>,
    }
);
