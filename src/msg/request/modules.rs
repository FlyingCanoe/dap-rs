request!(
    ModulesRequest {
        /// The index of the first module to return if omitted modules start at 0.
        start_module | "startModule": Option<u64>,

        /// The u64 of modules to return. If moduleCount is not specified or 0, all
        /// modules are returned.
        module_count | "moduleCount": Option<u64>,
    }
);
