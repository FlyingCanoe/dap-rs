dap_type_struct!(
    /// An ExceptionFilterOptions is used to specify an exception filter together with a condition for the setExceptionsFilter request.
    ExceptionFilterOptions {
        /// An optional expression for conditional exceptions.
        /// The exception will break into the debugger if the result of the condition is true.
        condition | "condition": Option<String>,
        /// ID of an exception filter returned by the 'exceptionBreakpointFilters' capability.
        filter_id | "filterId": String,
    }
);
