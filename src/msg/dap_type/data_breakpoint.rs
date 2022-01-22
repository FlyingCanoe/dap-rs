use crate::utils::{parse_optional_string, parse_string};

use super::DataBreakpointAccessType;

dap_type_struct!(
    DataBreakpoint {
        /// An id representing the data. This id is returned from the
        /// dataBreakpointInfo request.
        data_id | "dataId": String = parse_string,

        /// The access type of the data.
        access_type | "accessType": Option<DataBreakpointAccessType> = DataBreakpointAccessType::parse_optional,

        /// An optional expression for conditional breakpoints.
        condition | "condition": Option<String> = parse_optional_string,

        /// An optional expression that controls how many hits of the breakpoint are
        /// ignored.
        /// The backend is expected to interpret the expression as needed.
        hit_condition | "hitCondition": Option<String> = parse_optional_string,
    }
);
