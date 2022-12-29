dap_type_enum!(
    /// Logical areas that can be invalidated by the 'invalidated' event.
    InvalidatedAreas {
        Other,
        /// All previously fetched data has become invalid and needs to be refetched.
        All | "all",
        /// Previously fetched stack related data has become invalid and needs to be refetched.
        Stacks | "stacks",
        /// Previously fetched thread related data has become invalid and needs to be refetched.
        Threads | "threads",
        /// Previously fetched variable data has become invalid and needs to be refetched.
        Variables | "variables",
    }
);
