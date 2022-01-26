use crate::utils::{parse_optional_bool, parse_u64};

event!(
    ContinuedEvent {
        /// The thread which was continued.
        thread_id_ | "threadId": u64 = parse_u64,

        /// If 'allThreadsContinued' is true, a debug adapter can announce that all
        /// threads have continued.
        all_threads_continued | "allThreadsContinued": Option<bool> = parse_optional_bool,
    }
);
