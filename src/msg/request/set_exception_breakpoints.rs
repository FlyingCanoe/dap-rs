use crate::msg::dap_type::{ExceptionFilterOptions, ExceptionOptions};
use crate::utils::parse_string_vec;

request2!(
    SetExceptionBreakpointsRequest {
        /// Set of exception filters specified by their ID. The set of all possible
        /// exception filters is defined by the 'exceptionBreakpointFilters'
        /// capability. The 'filter' and 'filterOptions' sets are additive.
        filters | "filters": Vec<String> = parse_string_vec,

        /// Set of exception filters and their options. The set of all possible
        /// exception filters is defined by the 'exceptionBreakpointFilters'
        /// capability. This attribute is only honored by a debug adapter if the
        /// capability 'supportsExceptionFilterOptions' is true. The 'filter' and
        /// 'filterOptions' sets are additive.
        filter_options | "filterOptions": Option<Vec<ExceptionFilterOptions>> = ExceptionFilterOptions::parse_optional_vec,

        /// Configuration options for selected exceptions.
        /// The attribute is only honored by a debug adapter if the capability
        /// 'supportsExceptionOptions' is true.
        exception_options | "exceptionOptions": Option<Vec<ExceptionOptions>> = ExceptionOptions::parse_optional_vec,
    }
);
