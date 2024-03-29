﻿dap_type_enum!(
    /// Describes how the debug engine started debugging this process.
    StartMethod {
        /// Process was launched under the debugger.
        Launch | "launch",
        /// Debugger attached to an existing process.
        Attach | "attach",
        /// A project launcher component has launched a new process in a suspended state and then asked the debugger to attach.
        AttachForSuspendedLaunch | "attachForSuspendedLaunch",
    }
);

event!(
    /// The event indicates that the debugger has begun debugging a new process. Either one that it has launched, or one that it has attached to.
    ProcessEvent | "process" {
        /// The size of a pointer or address for this process, in bits. This value may be used by clients when formatting addresses for display.
        pointer_size | "pointerSize": Option<u64>,
        /// The system process id of the debugged process. This property will be missing for non-system processes.
        system_process_id | "systemProcessId": Option<u64>,
        /// The logical name of the process. This is usually the full path to process's executable file. Example: /home/example/myproj/program.js.
        name | "name": String,
        /// If true, the process is running on the same computer as the debug adapter.
        is_local_process | "isLocalProcess": Option<bool>,
        /// Describes how the debug engine started debugging this process.
        start_method | "startMethod": Option<StartMethod>,
    }
);
