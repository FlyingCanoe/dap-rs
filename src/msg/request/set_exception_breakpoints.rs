use crate::msg::dap_type::exception_filter_options::ExceptionFilterOptions;
use crate::msg::dap_type::exception_options::ExceptionOptions;
use crate::msg::dap_type::breakpoint::Breakpoint;

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

response!(
    /// Response to 'setExceptionBreakpoints' request.
    /// The response contains an array of Breakpoint objects with information about each exception breakpoint or filter. The Breakpoint objects are in the same order as the elements of the 'filters', 'filterOptions', 'exceptionOptions' arrays given as arguments. If both 'filters' and 'filterOptions' are given, the returned array must start with 'filters' information first, followed by 'filterOptions' information.
    /// The mandatory 'verified' property of a Breakpoint object signals whether the exception breakpoint or filter could be successfully created and whether the optional condition or hit count expressions are valid. In case of an error the 'message' property explains the problem. An optional 'id' property can be used to introduce a unique ID for the exception breakpoint or filter so that it can be updated subsequently by sending breakpoint events.
    /// For backward compatibility both the 'breakpoints' array and the enclosing 'body' are optional. If these elements are missing a client will not be able to show problems for individual exception breakpoints or filters.
    SetExceptionBreakpointsResponse {
        /// Information about the exception breakpoints or filters.
        /// The breakpoints returned are in the same order as the elements of the 'filters', 'filterOptions', 'exceptionOptions' arrays in the arguments. If both 'filters' and 'filterOptions' are given, the returned array must start with 'filters' information first, followed by 'filterOptions' information.
        breakpoints | "breakpoints": Option<Vec<Breakpoint>>,
    }
);
