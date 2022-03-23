use crate::msg::dap_type::exception_break_mode::ExceptionBreakMode;
use crate::msg::dap_type::exception_details::ExceptionDetails;

request!(
    /// Retrieves the details of the exception that caused this event to be raised.
    /// Clients should only call this request if the capability 'supportsExceptionInfoRequest' is true.
    ExceptionInfoRequest | "exceptionInfo" {
        /// Thread for which exception information should be retrieved.
        thread_id | "threadId": u64,
    }
);

response!(
    /// Response to 'exceptionInfo' request.
    ExceptionInfoResponse | "exceptionInfo" {
        /// Mode that caused the exception notification to be raised.
        break_mode | "breakMode": ExceptionBreakMode,
        /// Descriptive text for the exception provided by the debug adapter.
        description | "description": Option<String>,
        /// Detailed information about the exception.
        details | "details": Option<ExceptionDetails>,
        /// ID of the exception that was thrown.
        exception_id | "exceptionId": String,
    }
);
