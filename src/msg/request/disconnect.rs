request!(
    type Response = ();

    /// The 'disconnect' request is sent from the client to the debug adapter in order to stop debugging.
    /// It asks the debug adapter to disconnect from the debuggee and to terminate the debug adapter.
    /// If the debuggee has been started with the 'launch' request, the 'disconnect' request terminates the debuggee.
    /// If the 'attach' request was used to connect to the debuggee, 'disconnect' does not terminate the debuggee.
    /// This behavior can be controlled with the 'terminateDebuggee' argument (if supported by the debug adapter).
    DisconnectRequest | "disconnect" {
        /// A value of true indicates that this 'disconnect' request is part of a
        /// restart sequence.
        restart | "restart": Option<bool>,

        /// Indicates whether the debuggee should be terminated when the debugger is
        /// disconnected.
        /// If unspecified, the debug adapter is free to do whatever it thinks is best.
        /// The attribute is only honored by a debug adapter if the capability 'supportTerminateDebuggee' is true.
        terminate_debuggee | "terminateDebuggee": Option<bool>,
        /// Indicates whether the debuggee should stay suspended when the debugger is disconnected.
        /// If unspecified, the debuggee should resume execution.
        /// The attribute is only honored by a debug adapter if the capability 'supportSuspendDebuggee' is true.
        suspend_debuggee | "suspendDebuggee": Option<bool>,
        /// A value of true indicates that this 'disconnect' request is part of a restart sequence.
        restart | "restart": Option<bool>,
    }
);
