use crate::msg::dap_type::StackFrameFormat;

request!(
    /// The request returns a stacktrace from the current execution state of a given thread.
    /// A client can request all stack frames by omitting the startFrame and levels arguments. For performance conscious clients and if the debug adapter's 'supportsDelayedStackTraceLoading' capability is true, stack frames can be retrieved in a piecemeal way with the startFrame and levels arguments. The response of the stackTrace request may contain a totalFrames property that hints at the total number of frames in the stack. If a client needs this total number upfront, it can issue a request for a single (first) frame and depending on the value of totalFrames decide how to proceed. In any case a client should be prepared to receive less frames than requested, which is an indication that the end of the stack has been reached.
    StackTraceRequest {
        /// Retrieve the stacktrace for this thread.
        thread_id | "threadId": u64,
        /// Specifies details on how to format the stack frames.
        /// The attribute is only honored by a debug adapter if the capability 'supportsValueFormattingOptions' is true.
        format | "format": Option<StackFrameFormat>,
        /// The maximum number of frames to return. If levels is not specified or 0, all frames are returned.
        levels | "levels": Option<u64>,
        /// The index of the first frame to return; if omitted frames start at 0.
        start_frame | "startFrame": Option<u64>,
    }
);
