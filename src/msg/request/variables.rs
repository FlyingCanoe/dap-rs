use crate::msg::dap_type::{ValueFormat, VariablesFilter};
use crate::utils::{parse_optional_u64, parse_u64};

request!(
    VariablesRequest {
        /// The Variable reference.
        variables_reference | "variablesReference": u64 = parse_u64,

        /// Optional filter to limit the child variables to either named or indexed. If
        /// omitted, both types are fetched.
        /// Values: 'indexed', 'named', etc.
        filter | "filter": Option<VariablesFilter> = VariablesFilter::parse_option,

        /// The index of the first variable to return if omitted children start at 0.
        start | "start": Option<u64> = parse_optional_u64,

        /// The u64 of variables to return. If count is missing or 0, all variables
        /// are returned.
        count | "count": Option<u64> = parse_optional_u64,

        /// Specifies details on how to format the Variable values.
        /// The attribute is only honored by a debug adapter if the capability
        /// 'supportsValueFormattingOptions' is true.
        format | "format": Option<ValueFormat> = ValueFormat::parse_option,
    }
);
