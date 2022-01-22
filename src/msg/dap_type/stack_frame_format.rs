use crate::utils::parse_optional_bool;

dap_type_struct!(
    StackFrameFormat {
        /// Displays parameters for the stack frame.
        parameters | "parameters": Option<bool> = parse_optional_bool,

        /// Displays the types of parameters for the stack frame.
        parameter_types | "parameterTypes": Option<bool> = parse_optional_bool,

        /// Displays the names of parameters for the stack frame.
        parameter_names | "parameterNames": Option<bool> = parse_optional_bool,

        /// Displays the values of parameters for the stack frame.
        parameter_values | "parameterValues": Option<bool> = parse_optional_bool,

        /// Displays the line u64 of the stack frame.
        line | "line": Option<bool> = parse_optional_bool,

        /// Displays the module of the stack frame.
        module | "module": Option<bool> = parse_optional_bool,

        /// Includes all stack frames, including those the debug adapter might
        /// otherwise hide.
        include_all | "includeAll": Option<bool> = parse_optional_bool,
    }
);
