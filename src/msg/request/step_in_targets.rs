request!(
    StepInTargetsRequest {
        /// The stack frame for which to retrieve the possible stepIn targets.
        frame_id | "frameId": u64,
    }
);
