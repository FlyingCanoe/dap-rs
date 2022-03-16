use crate::msg::dap_type::breakpoint::Breakpoint;

dap_type_enum!(
    /// The reason for the event.
    Reason {
        Other,
        Changed | "changed",
        New | "new",
        Removed | "removed",
    }
);

event!(
    /// The event indicates that some information about a breakpoint has changed.
    BreakpointEvent {
        /// The 'id' attribute is used to find the target breakpoint and the other attributes are used as the new values.
        breakpoint | "breakpoint": Breakpoint,
        /// The reason for the event.
        reason | "reason": Reason,
    }
);
