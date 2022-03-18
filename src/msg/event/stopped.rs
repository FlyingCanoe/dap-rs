dap_type_enum!(
    /// The reason for the event.
    /// For backward compatibility this string is shown in the UI if the 'description' attribute is missing (but it must not be translated).
    Reason {
        Other,
        Step | "step",
        Breakpoint | "breakpoint",
        Exception | "exception",
        Pause | "pause",
        Entry | "entry",
        Goto | "goto",
        FunctionBreakpoint | "function breakpoint",
        DataBreakpoint | "data breakpoint",
        InstructionBreakpoint | "instruction breakpoint",
    }
);

event!(
    /// The event indicates that the execution of the debuggee has stopped due to some condition.
    /// This can be caused by a break point previously set, a stepping request has completed, by executing a debugger statement etc.
    StoppedEvent {
        /// Ids of the breakpoints that triggered the event. In most cases there will be only a single breakpoint but here are some examples for multiple breakpoints:
        /// - Different types of breakpoints map to the same location.
        /// - Multiple source breakpoints get collapsed to the same instruction by the compiler/runtime.
        /// - Multiple function breakpoints with different function names map to the same location.
        hit_breakpoint_ids | "hitBreakpointIds": Option<Vec<u64>>,
        /// The full reason for the event, e.g. 'Paused on exception'. This string is shown in the UI as is and must be translated.
        description | "description": Option<String>,
        /// Additional information. E.g. if reason is 'exception', text contains the exception name. This string is shown in the UI.
        text | "text": Option<String>,
        /// A value of true hints to the frontend that this event should not change the focus.
        preserve_focus_hint | "preserveFocusHint": Option<bool>,
        /// If 'allThreadsStopped' is true, a debug adapter can announce that all threads have stopped.
        /// - The client should use this information to enable that all threads can be expanded to access their stacktraces.
        /// - If the attribute is missing or false, only the thread with the given threadId can be expanded.
        all_threads_stopped | "allThreadsStopped": Option<bool>,
        /// The reason for the event.
        /// For backward compatibility this string is shown in the UI if the 'description' attribute is missing (but it must not be translated).
        reason | "reason": Reason,
        /// The thread which was stopped.
        thread_id | "threadId": Option<u64>,
    }
);
