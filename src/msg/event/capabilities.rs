﻿use crate::msg::dap_type::capabilities::Capabilities;

event!(
    /// The event indicates that one or more capabilities have changed.
    /// Since the capabilities are dependent on the frontend and its UI, it might not be possible to change that at random times (or too late).
    /// Consequently this event has a hint characteristic: a frontend can only be expected to make a 'best effort' in honouring individual capabilities but there are no guarantees.
    /// Only changed capabilities need to be included, all other capabilities keep their values.
    CapabilitiesEvent | "capabilities" {
        /// The set of updated capabilities.
        capabilities | "capabilities": Capabilities,
    }
);
