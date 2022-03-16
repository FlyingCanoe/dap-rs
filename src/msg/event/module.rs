use crate::msg::dap_type::module::Module;

dap_type_enum!(
    /// The reason for the event.
    Reason {
        New | "new",
        Changed | "changed",
        Removed | "removed",
    }
);

event!(
    /// The event indicates that some information about a module has changed.
    ModuleEvent {
        /// The new, changed, or removed module. In case of 'removed' only the module id is used.
        module | "module": Module,
        /// The reason for the event.
        reason | "reason": Reason,
    }
);
