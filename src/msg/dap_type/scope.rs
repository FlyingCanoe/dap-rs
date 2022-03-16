use crate::msg::dap_type::source::Source;

dap_type_enum!(
    /// An optional hint for how to present this scope in the UI. If this attribute is missing, the scope is shown with a generic UI.
    PresentationHint {
        Other,
        /// Scope contains method arguments.
        Arguments | "arguments",
        /// Scope contains local variables.
        Locals | "locals",
        /// Scope contains registers. Only a single 'registers' scope should be returned from a 'scopes' request.
        Registers | "registers",
    }
);

dap_type_struct!(
    /// A Scope is a named container for variables. Optionally a scope can map to a source or a range within a source.
    Scope {
        /// Optional source for this scope.
        source | "source": Option<Source>,
        /// The number of indexed variables in this scope.
        /// The client can use this optional information to present the variables in a paged UI and fetch them in chunks.
        indexed_variables | "indexedVariables": Option<u64>,
        /// If true, the number of variables in this scope is large or expensive to retrieve.
        expensive | "expensive": bool,
        /// Optional start line of the range covered by this scope.
        line | "line": Option<u64>,
        /// The variables of this scope can be retrieved by passing the value of variablesReference to the VariablesRequest.
        variables_reference | "variablesReference": u64,
        /// Name of the scope such as 'Arguments', 'Locals', or 'Registers'. This string is shown in the UI as is and can be translated.
        name | "name": String,
        /// Optional start column of the range covered by this scope.
        column | "column": Option<u64>,
        /// An optional hint for how to present this scope in the UI. If this attribute is missing, the scope is shown with a generic UI.
        presentation_hint | "presentationHint": Option<PresentationHint>,
        /// Optional end column of the range covered by this scope.
        end_column | "endColumn": Option<u64>,
        /// Optional end line of the range covered by this scope.
        end_line | "endLine": Option<u64>,
        /// The number of named variables in this scope.
        /// The client can use this optional information to present the variables in a paged UI and fetch them in chunks.
        named_variables | "namedVariables": Option<u64>,
    }
);
