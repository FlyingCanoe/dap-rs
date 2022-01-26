use crate::utils::{
    parse_bool, parse_optional_bool, parse_optional_string, parse_optional_u64,
    parse_optional_u64_vec, parse_string, parse_string_vec, parse_u64,
};

use crate::msg::dap_type::Capabilities;

dap_type_struct!(
    CapabilitiesEvent {
        /// The set of updated capabilities.
        capabilities | "capabilities": Capabilities = Capabilities::parse,
    }
);
