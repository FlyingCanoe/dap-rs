use crate::msg::dap_type::Capabilities;

event!(
    CapabilitiesEvent {
        /// The set of updated capabilities.
        capabilities | "capabilities": Capabilities = Capabilities::parse,
    }
);
