use crate::utils::parse_u64;

event!(
    ExitedEvent {
        /// The exit code returned from the debuggee.
        exit_code | "exitCode": u64 = parse_u64,
    }
);
