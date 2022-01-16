request!(
    ModulesRequest {
        optional_args = false;
        u64 {},
        Option<u64> {
            /// The index of the first module to return if omitted modules start at 0.
            start_module: "startModule",
            /// The number of modules to return. If moduleCount is not specified or 0, all
            /// modules are returned.
            module_count: "moduleCount",
        },
        Option<bool> {},
        String {},
        Option<String> {},
        Option<json::Value> {},
        Custom {},
        Option<Custom> {},
    }
);
