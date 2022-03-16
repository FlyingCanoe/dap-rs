dap_type_struct!(
    /// Properties of a breakpoint location returned from the 'breakpointLocations' request.
    BreakpointLocation {
        /// Optional end line of breakpoint location if the location covers a range.
        end_line | "endLine": Option<u64>,
        /// Optional start column of breakpoint location.
        column | "column": Option<u64>,
        /// Start line of breakpoint location.
        line | "line": u64,
        /// Optional end column of breakpoint location if the location covers a range.
        end_column | "endColumn": Option<u64>,
    }
);
