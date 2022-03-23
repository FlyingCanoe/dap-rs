use crate::msg::dap_type::source::Source;

dap_type_enum!(
    /// The reason for the event.
    Reason {
        New | "new",
        Changed | "changed",
        Removed | "removed",
    }
);

event!(
    /// The event indicates that some source has been added, changed, or removed from the set of all loaded sources.
    LoadedSourceEvent | "loadedSource" {
        /// The reason for the event.
        reason | "reason": Reason,
        /// The new, changed, or removed source.
        source | "source": Source,
    }
);
