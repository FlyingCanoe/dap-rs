use crate::msg::dap_type::Capabilities;

dap_type_struct!(
    CapabilitiesEvent {
        /// The set of updated capabilities.
        capabilities | "capabilities": Capabilities,
    }
);
