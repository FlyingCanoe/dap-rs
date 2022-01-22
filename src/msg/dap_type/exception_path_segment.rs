use crate::utils::{parse_optional_bool, parse_string_vec};

dap_type_struct!(
    ExceptionPathSegment {
        /// If false or missing this segment matches the names provided, otherwise it
        /// matches anything except the names provided.
        negate | "negate": Option<bool> = parse_optional_bool,

        /// Depending on the value of 'negate' the names that should match or not
        /// match.
        names | "names": Vec<String> = parse_string_vec,
    }
);
