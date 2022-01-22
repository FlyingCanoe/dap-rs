use crate::utils::parse_u64;

request!(
    StepInTargetsRequest {
        /// The stack frame for which to retrieve the possible stepIn targets.
        frame_id | "frameId": u64 = parse_u64,
    }
);
