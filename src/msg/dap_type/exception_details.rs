dap_type_struct!(
    /// Detailed information about an exception that has occurred.
    ExceptionDetails {
        /// Message contained in the exception.
        message | "message": Option<String>,
        /// Stack trace at the time the exception was thrown.
        stack_trace | "stackTrace": Option<String>,
        /// Optional expression that can be evaluated in the current scope to obtain the exception object.
        evaluate_name | "evaluateName": Option<String>,
        /// Fully-qualified type name of the exception object.
        full_type_name | "fullTypeName": Option<String>,
        /// Short type name of the exception object.
        type_name | "typeName": Option<String>,
        /// Details of the exception contained by this exception, if any.
        inner_exception | "innerException": Option<Vec<ExceptionDetails>>,
    }
);
