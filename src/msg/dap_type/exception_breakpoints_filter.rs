dap_type_struct!(
    /// An ExceptionBreakpointsFilter is shown in the UI as an filter option for configuring how exceptions are dealt with.
    ExceptionBreakpointsFilter {
        /// Controls whether a condition can be specified for this filter option. If false or missing, a condition can not be set.
        supports_condition | "supportsCondition": Option<bool>,
        /// An optional help text providing additional information about the exception filter. This string is typically shown as a hover and must be translated.
        description | "description": Option<String>,
        /// The internal ID of the filter option. This value is passed to the 'setExceptionBreakpoints' request.
        filter | "filter": String,
        /// Initial value of the filter option. If not specified a value 'false' is assumed.
        default | "default": Option<bool>,
        /// The name of the filter option. This will be shown in the UI.
        label | "label": String,
        /// An optional help text providing information about the condition. This string is shown as the placeholder text for a text box and must be translated.
        condition_description | "conditionDescription": Option<String>,
    }
);
