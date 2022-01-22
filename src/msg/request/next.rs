use crate::msg::dap_type::SteppingGranularity;
use crate::utils::{parse_optional_bool, parse_u64};

request!(
    NextRequest {
        /// Specifies the thread for which to resume execution for one step (of the
        /// given granularity).
        thread_id | "threadId": u64 = parse_u64,

        /// If this optional flag is true, all other suspended threads are not resumed.
        single_thread | "singleThread": Option<bool> = parse_optional_bool,

        /// Optional granularity to step. If no granularity is specified, a granularity
        /// of 'statement' is assumed.
        granularity | "granularity": Option<SteppingGranularity> = SteppingGranularity::parse_optional,
    }
);
