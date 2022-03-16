use crate::msg::dap_type::ExceptionFilterOptions;
use crate::msg::dap_type::ExceptionOptions;

request!(
    /// The request configures the debuggers response to thrown exceptions.
    /// If an exception is configured to break, a 'stopped' event is fired (with reason 'exception').
    /// Clients should only call this request if the capability 'exceptionBreakpointFilters' returns one or more filters.
    SetExceptionBreakpointsRequest {
        /// Set of exception filters and their options. The set of all possible exception filters is defined by the 'exceptionBreakpointFilters' capability. This attribute is only honored by a debug adapter if the capability 'supportsExceptionFilterOptions' is true. The 'filter' and 'filterOptions' sets are additive.
        filter_options | "filterOptions": Option<Vec<ExceptionFilterOptions>>,
        /// Configuration options for selected exceptions.
        /// The attribute is only honored by a debug adapter if the capability 'supportsExceptionOptions' is true.
        exception_options | "exceptionOptions": Option<Vec<ExceptionOptions>>,
        /// Set of exception filters specified by their ID. The set of all possible exception filters is defined by the 'exceptionBreakpointFilters' capability. The 'filter' and 'filterOptions' sets are additive.
        filters | "filters": Vec<String>,
    }
);
