request!(

    DisconnectRequest {
        optional_args = true;
        u64 {},
        Option<u64> {},
        Option<bool> {
            /// A value of true indicates that this 'disconnect' request is part of a
            /// restart sequence.
            restart: "restart",
            /// Indicates whether the debuggee should be terminated when the debugger is
            /// disconnected.
            /// If unspecified, the debug adapter is free to do whatever it thinks is best.
            /// The attribute is only honored by a debug adapter if the capability
            /// 'supportTerminateDebuggee' is true.
            terminate_debuggee: "terminateDebuggee",
            /// Indicates whether the debuggee should stay suspended when the debugger is
            /// disconnected.
            /// If unspecified, the debuggee should resume execution.
            /// The attribute is only honored by a debug adapter if the capability
            /// 'supportSuspendDebuggee' is true.
            suspend_debuggee: "suspendDebuggee",
        },
        String {},
        Custom {},
    }
);
