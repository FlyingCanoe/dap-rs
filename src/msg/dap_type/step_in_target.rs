dap_type_struct!(
    /// A StepInTarget can be used in the 'stepIn' request and determines into which single target the stepIn request should step.
    StepInTarget {
        /// Unique identifier for a stepIn target.
        id | "id": u64,
        /// The name of the stepIn target (shown in the UI).
        label | "label": String,
    }
);
