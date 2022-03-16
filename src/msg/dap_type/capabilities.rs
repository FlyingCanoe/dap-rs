use super::{ChecksumAlgorithm, ColumnDescriptor, ExceptionBreakpointsFilter};

dap_type_struct!(
    /// Information about the capabilities of a debug adapter.
    Capabilities {
        /// The debug adapter supports the 'configurationDone' request.
        supports_configuration_done_request | "supportsConfigurationDoneRequest": Option<bool>,

        /// The debug adapter supports function breakpoints.
        supports_function_breakpoints | "supportsFunctionBreakpoints": Option<bool>,

        /// The debug adapter supports conditional breakpoints.
        supports_conditional_breakpoints | "supportsConditionalBreakpoints": Option<bool>,

        /// The debug adapter supports breakpoints that break execution after a
        /// specified u64 of hits.
        supports_hit_conditional_breakpoints | "supportsHitConditionalBreakpoints": Option<bool>,

        /// The debug adapter supports a (side effect free) evaluate request for data
        /// hovers.
        supports_evaluate_for_hovers | "supportsEvaluateForHovers": Option<bool>,

        /// Available exception filter options for the 'setExceptionBreakpoints'
        /// request.
        exception_breakpoint_filters | "exceptionBreakpointFilters": Option<Vec<ExceptionBreakpointsFilter>>,

        /// The debug adapter supports stepping back via the 'stepBack' and
        /// 'reverseContinue' requests.
        supports_step_back | "supportsStepBack": Option<bool>,

        /// The debug adapter supports setting a variable to a value.
        supports_set_variable | "supportsSetVariable": Option<bool>,

        /// The debug adapter supports restarting a frame.
        supports_restart_frame | "supportsRestartFrame": Option<bool>,

        /// The debug adapter supports the 'gotoTargets' request.
        supports_goto_targets_request | "supportsGotoTargetsRequest": Option<bool>,

        /// The debug adapter supports the 'stepInTargets' request.
        supports_step_in_targets_request | "supportsStepInTargetsRequest": Option<bool>,

        /// The debug adapter supports the 'completions' request.
        supports_completions_request | "supportsCompletionsRequest": Option<bool>,

        /// The set of characters that should trigger completion in a REPL. If not
        /// specified, the UI should assume the '.' character.
        completion_trigger_characters | "completionTriggerCharacters": Option<Vec<String>>,

        /// The debug adapter supports the 'modules' request.
        supports_modules_request | "supportsModulesRequest": Option<bool>,

        /// The set of additional module information exposed by the debug adapter.
        additional_module_columns | "additionalModuleColumns": Option<Vec<ColumnDescriptor>>,

        /// Checksum algorithms supported by the debug adapter.
        supported_checksum_algorithms | "supportedChecksumAlgorithms": Option<Vec<ChecksumAlgorithm>>,

        /// The debug adapter supports the 'restart' request. In this case a client
        /// should not implement 'restart' by terminating and relaunching the adapter
        /// but by calling the RestartRequest.
        supports_restart_request | "supportsRestartRequest": Option<bool>,

        /// The debug adapter supports 'exceptionOptions' on the
        /// setExceptionBreakpoints request.
        supports_exception_options | "supportsExceptionOptions": Option<bool>,

        /// The debug adapter supports a 'format' attribute on the stackTraceRequest,
        /// variablesRequest, and evaluateRequest.
        supports_value_formatting_options | "supportsValueFormattingOptions": Option<bool>,

        /// The debug adapter supports the 'exceptionInfo' request.
        supports_exception_info_request | "supportsExceptionInfoRequest": Option<bool>,

        /// The debug adapter supports the 'terminateDebuggee' attribute on the
        /// 'disconnect' request.
        support_terminate_debuggee | "supportTerminateDebuggee": Option<bool>,

        /// The debug adapter supports the 'suspendDebuggee' attribute on the
        /// 'disconnect' request.
        support_suspend_debuggee | "supportSuspendDebuggee": Option<bool>,

        /// The debug adapter supports the delayed loading of parts of the stack, which
        /// requires that both the 'startFrame' and 'levels' arguments and an optional
        /// 'totalFrames' result of the 'StackTrace' request are supported.
        supports_delayed_stack_trace_loading | "supportsDelayedStackTraceLoading": Option<bool>,

        /// The debug adapter supports the 'loadedSources' request.
        supports_loaded_sources_request | "supportsLoadedSourcesRequest": Option<bool>,

        /// The debug adapter supports logpoints by interpreting the 'logMessage'
        /// attribute of the SourceBreakpoint.
        supports_log_points | "supportsLogPoints": Option<bool>,

        /// The debug adapter supports the 'terminateThreads' request.
        supports_terminate_threads_request | "supportsTerminateThreadsRequest": Option<bool>,

        /// The debug adapter supports the 'setExpression' request.
        supports_set_expression | "supportsSetExpression": Option<bool>,

        /// The debug adapter supports the 'terminate' request.
        supports_terminate_request | "supportsTerminateRequest": Option<bool>,

        /// The debug adapter supports data breakpoints.
        supports_data_breakpoints | "supportsDataBreakpoints": Option<bool>,

        /// The debug adapter supports the 'readMemory' request.
        supports_read_memory_request | "supportsReadMemoryRequest": Option<bool>,

        /// The debug adapter supports the 'writeMemory' request.
        supports_write_memory_request | "supportsWriteMemoryRequest": Option<bool>,

        /// The debug adapter supports the 'disassemble' request.
        supports_disassemble_request | "supportsDisassembleRequest": Option<bool>,

        /// The debug adapter supports the 'cancel' request.
        supports_cancel_request | "supportsCancelRequest": Option<bool>,

        /// The debug adapter supports the 'breakpointLocations' request.
        supports_breakpoint_locations_request | "supportsBreakpointLocationsRequest": Option<bool>,

        /// The debug adapter supports the 'clipboard' context value in the 'evaluate'
        /// request.
        supports_clipboard_context | "supportsClipboardContext": Option<bool>,

        /// The debug adapter supports stepping granularities (argument 'granularity')
        /// for the stepping requests.
        supports_stepping_granularity | "supportsSteppingGranularity": Option<bool>,

        /// The debug adapter supports adding breakpoints based on instruction
        /// references.
        supports_instruction_breakpoints | "supportsInstructionBreakpoints": Option<bool>,

        /// The debug adapter supports 'filterOptions' as an argument on the
        /// 'setExceptionBreakpoints' request.
        supports_exception_filter_options | "supportsExceptionFilterOptions": Option<bool>,

        /// The debug adapter supports the 'singleThread' property on the execution
        /// requests ('continue', 'next', 'stepIn', 'stepOut', 'reverseContinue',
        /// 'stepBack').
        supports_single_thread_execution_requests | "supportsSingleThreadExecutionRequests": Option<bool>,
    }
);
