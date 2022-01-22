use crate::msg::dap_type::StackFrameFormat;
use crate::utils::{parse_optional_u64, parse_u64};

request!(
    StackTraceRequest {
        /// Retrieve the stacktrace for this thread.
        thread_id | "threadId": u64 = parse_u64,

        /// The index of the first frame to return if omitted frames start at 0.
        start_frame | "startFrame": Option<u64> = parse_optional_u64,

        /// The maximum u64 of frames to return. If levels is not specified or 0,
        /// all frames are returned.
        levels | "levels": Option<u64> = parse_optional_u64,

        /// Specifies details on how to format the stack frames.
        /// The attribute is only honored by a debug adapter if the capability
        /// 'supportsValueFormattingOptions' is true.
        format | "format": Option<StackFrameFormat> = StackFrameFormat::parse_optional,
    }
);
