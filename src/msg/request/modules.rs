request!(
    type Response = ModulesResponse;

    /// Modules can be retrieved from the debug adapter with this request which can either return all modules or a range of modules to support paging.
    /// Clients should only call this request if the capability 'supportsModulesRequest' is true.
    ModulesRequest | "modules" {
        /// The index of the first module to return if omitted modules start at 0.
        start_module | "startModule": Option<u64>,
        /// The number of modules to return. If moduleCount is not specified or 0, all modules are returned.
        module_count | "moduleCount": Option<u64>,
    }
);

response!(
    /// Response to 'modules' request.
    ModulesResponse | "modules" {
        /// The total number of modules available.
        total_modules | "totalModules": Option<u64>,
        /// All modules or range of modules.
        modules | "modules": Vec<Module>,
    }
);
