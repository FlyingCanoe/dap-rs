request!(
    /// This request retrieves the possible stepIn targets for the specified stack frame.
    /// These targets can be used in the 'stepIn' request.
    /// The StepInTargets may only be called if the 'supportsStepInTargetsRequest' capability exists and is true.
    /// Clients should only call this request if the capability 'supportsStepInTargetsRequest' is true.
    StepInTargetsRequest {
        /// The stack frame for which to retrieve the possible stepIn targets.
        frame_id | "frameId": u64,
    }
);
