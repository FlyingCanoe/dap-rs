dap_type_enum!(
    LoadedSourceReason {
        New | "new",
        Changed | "changed",
        Removed | "removed",
    }
);

use crate::msg::dap_type::Source;

event!(
    LoadedSourceEvent {
        /// The reason for the event.
        /// Values: 'new', 'changed', 'removed', etc.
        reason | "reason": LoadedSourceReason = LoadedSourceReason::parse,

        /// The new, changed, or removed source.
        source_ | "source": Source = Source::parse,
    }
);
