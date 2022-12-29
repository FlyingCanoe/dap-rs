dap_type_enum!(
    /// This enumeration defines all possible conditions when a thrown exception should result in a break.
    /// never: never breaks,
    /// always: always breaks,
    /// unhandled: breaks when exception unhandled,
    /// userUnhandled: breaks if the exception is not handled by user code.
    ExceptionBreakMode {
        Never | "never",
        Always | "always",
        Unhandled | "unhandled",
        UserUnhandled | "userUnhandled",
    }
);
