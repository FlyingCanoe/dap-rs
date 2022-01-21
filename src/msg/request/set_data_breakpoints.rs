use crate::msg::dap_type::DataBreakpoint;

request!(
    SetDataBreakpointsRequest {
        optional_args = false;
        u64 {},
        Option<u64> {},
        Option<Vec<u64>> {},
        Option<bool> {},
        String {},
        Option<String> {},
        Option<json::Value> {},
        Custom {
            {
                type = Vec<DataBreakpoint>;
                closure = DataBreakpoint::parse_vec;
                ///  The contents of this array replaces all existing data breakpoints. An empty
                ///  array clears all data breakpoints.
                format: "format";
            },
        },
    }
);
