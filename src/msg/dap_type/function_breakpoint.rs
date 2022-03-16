﻿dap_type_struct!(
    /// Properties of a breakpoint passed to the setFunctionBreakpoints request.
    FunctionBreakpoint {
        /// An optional expression that controls how many hits of the breakpoint are ignored.
        /// The backend is expected to interpret the expression as needed.
        /// The attribute is only honored by a debug adapter if the capability 'supportsHitConditionalBreakpoints' is true.
        hit_condition | "hitCondition": Option<String>,
        /// An optional expression for conditional breakpoints.
        /// It is only honored by a debug adapter if the capability 'supportsConditionalBreakpoints' is true.
        condition | "condition": Option<String>,
        /// The name of the function.
        name | "name": String,
    }
);
