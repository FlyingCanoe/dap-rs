use crate::utils::{parse_optional_string, parse_optional_u64, parse_u64};

dap_type_struct!(
    SourceBreakpoint {
        /// The source line of the breakpoint or logpoint.
        line | "line": u64 = parse_u64,

        /// An optional source column of the breakpoint.
        column | "column": Option<u64> = parse_optional_u64,

        /// An optional expression for conditional breakpoints.
        /// It is only honored by a debug adapter if the capability
        /// 'supportsConditionalBreakpoints' is true.
        condition | "condition": Option<String> = parse_optional_string,

        /// An optional expression that controls how many hits of the breakpoint are
        /// ignored.
        /// The backend is expected to interpret the expression as needed.
        /// The attribute is only honored by a debug adapter if the capability
        /// 'supportsHitConditionalBreakpoints' is true.
        hit_condition | "hitCondition": Option<String> = parse_optional_string,

        /// If this attribute exists and is non-empty, the backend must not 'break'
        /// (stop)
        /// but log the message instead. Expressions within  are interpolated.
        /// The attribute is only honored by a debug adapter if the capability
        /// 'supportsLogPoints' is true.
        log_message | "logMessage": Option<String> = parse_optional_string,
    }
);
