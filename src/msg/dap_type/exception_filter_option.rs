dap_type_struct!(
    ExceptionFilterOptions {
        /// ID of an exception filter returned by the 'exceptionBreakpointFilters'
        /// capability.
        filter_id | "filterId": String,

        /// An optional expression for conditional exceptions.
        /// The exception will break into the debugger if the result of the condition
        /// is true.
        condition | "condition": Option<String>,
    }
);
