use crate::utils::{parse_optional_bool, parse_optional_string, parse_string};

dap_type_struct!(
    ExceptionBreakpointsFilter {
        /// The internal ID of the filter option. This value is passed to the
        /// 'setExceptionBreakpoints' request.
        filter | "filter": String = parse_string,

        /// The name of the filter option. This will be shown in the UI.
        label | "label": String = parse_string,

        /// An optional help text providing additional information about the exception
        /// filter. This String is typically shown as a hover and must be translated.
        description | "description": Option<String> = parse_optional_string,

        /// Initial value of the filter option. If not specified a value 'false' is
        /// assumed.
        default | "default": Option<bool> = parse_optional_bool,

        /// Controls whether a condition can be specified for this filter option. If
        /// false or missing, a condition can not be set.
        supports_condition | "supportsCondition": Option<bool> = parse_optional_bool,

        /// An optional help text providing information about the condition. This
        /// String is shown as the placeholder text for a text box and must be
        /// translated.
        condition_description | "conditionDescription": Option<String> = parse_optional_string,
    }
);