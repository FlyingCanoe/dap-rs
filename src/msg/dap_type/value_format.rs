use crate::utils::parse_optional_bool;

dap_type_struct!(
    ValueFormat {
        /// Display the value in hex.
        hex | "hex": Option<bool> = parse_optional_bool,
    }
);
