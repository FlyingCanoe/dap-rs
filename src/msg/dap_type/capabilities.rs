use super::{ColumnDescriptor, ExceptionBreakpointsFilter};
use crate::utils::{parse_optional_bool, parse_optional_string_vec};

use super::ChecksumAlgorithm;

dap_type_struct!(
    Capabilities {
        /// The debug adapter supports the 'configurationDone' request.
        supports_configuration_done_request | "supportsConfigurationDoneRequest": Option<bool> = parse_optional_bool,

        /// The debug adapter supports function breakpoints.
        supportsFunctionBreakpoints | "supportsFunctionBreakpoints": Option<bool> = parse_optional_bool,

        /// The debug adapter supports conditional breakpoints.
        supportsConditionalBreakpoints | "supportsConditionalBreakpoints": Option<bool> = parse_optional_bool,

        /// The debug adapter supports breakpoints that break execution after a
        /// specified u64 of hits.
        supportsHitConditionalBreakpoints | "supportsHitConditionalBreakpoints": Option<bool> = parse_optional_bool,

        /// The debug adapter supports a (side effect free) evaluate request for data
        /// hovers.
        supportsEvaluateForHovers | "supportsEvaluateForHovers": Option<bool> = parse_optional_bool,

        /// Available exception filter options for the 'setExceptionBreakpoints'
        /// request.
        exceptionBreakpointFilters | "exceptionBreakpointFilters": Option<Vec<ExceptionBreakpointsFilter>> = ExceptionBreakpointsFilter::parse_optional_vec,

        /// The debug adapter supports stepping back via the 'stepBack' and
        /// 'reverseContinue' requests.
        supportsStepBack | "supportsStepBack": Option<bool> = parse_optional_bool,

        /// The debug adapter supports setting a variable to a value.
        supportsSetVariable | "supportsSetVariable": Option<bool> = parse_optional_bool,

        /// The debug adapter supports restarting a frame.
        supportsRestartFrame | "supportsRestartFrame": Option<bool> = parse_optional_bool,

        /// The debug adapter supports the 'gotoTargets' request.
        supportsGotoTargetsRequest | "supportsGotoTargetsRequest": Option<bool> = parse_optional_bool,

        /// The debug adapter supports the 'stepInTargets' request.
        supportsStepInTargetsRequest | "supportsStepInTargetsRequest": Option<bool> = parse_optional_bool,

        /// The debug adapter supports the 'completions' request.
        supportsCompletionsRequest | "supportsCompletionsRequest": Option<bool> = parse_optional_bool,

        /// The set of characters that should trigger completion in a REPL. If not
        /// specified, the UI should assume the '.' character.
        completionTriggerCharacters | "completionTriggerCharacters": Option<Vec<String>> = parse_optional_string_vec,

        /// The debug adapter supports the 'modules' request.
        supportsModulesRequest | "supportsModulesRequest": Option<bool> = parse_optional_bool,

        /// The set of additional module information exposed by the debug adapter.
        additionalModuleColumns | "additionalModuleColumns": Option<Vec<ColumnDescriptor>> = ColumnDescriptor::parse_optional_vec,

        /// Checksum algorithms supported by the debug adapter.
        supportedChecksumAlgorithms | "supportedChecksumAlgorithms": Option<Vec<ChecksumAlgorithm>> = ChecksumAlgorithm::parse_optional_vec,

        /// The debug adapter supports the 'restart' request. In this case a client
        /// should not implement 'restart' by terminating and relaunching the adapter
        /// but by calling the RestartRequest.
        supportsRestartRequest | "supportsRestartRequest": Option<bool> = parse_optional_bool,

        /// The debug adapter supports 'exceptionOptions' on the
        /// setExceptionBreakpoints request.
        supportsExceptionOptions | "supportsExceptionOptions": Option<bool> = parse_optional_bool,

        /// The debug adapter supports a 'format' attribute on the stackTraceRequest,
        /// variablesRequest, and evaluateRequest.
        supportsValueFormattingOptions | "supportsValueFormattingOptions": Option<bool> = parse_optional_bool,

        /// The debug adapter supports the 'exceptionInfo' request.
        supportsExceptionInfoRequest | "supportsExceptionInfoRequest": Option<bool> = parse_optional_bool,

        /// The debug adapter supports the 'terminateDebuggee' attribute on the
        /// 'disconnect' request.
        supportTerminateDebuggee | "supportTerminateDebuggee": Option<bool> = parse_optional_bool,

        /// The debug adapter supports the 'suspendDebuggee' attribute on the
        /// 'disconnect' request.
        supportSuspendDebuggee | "supportSuspendDebuggee": Option<bool> = parse_optional_bool,

        /// The debug adapter supports the delayed loading of parts of the stack, which
        /// requires that both the 'startFrame' and 'levels' arguments and an optional
        /// 'totalFrames' result of the 'StackTrace' request are supported.
        supportsDelayedStackTraceLoading | "supportsDelayedStackTraceLoading": Option<bool> = parse_optional_bool,

        /// The debug adapter supports the 'loadedSources' request.
        supportsLoadedSourcesRequest | "supportsLoadedSourcesRequest": Option<bool> = parse_optional_bool,

        /// The debug adapter supports logpoints by interpreting the 'logMessage'
        /// attribute of the SourceBreakpoint.
        supportsLogPoints | "supportsLogPoints": Option<bool> = parse_optional_bool,

        /// The debug adapter supports the 'terminateThreads' request.
        supportsTerminateThreadsRequest | "supportsTerminateThreadsRequest": Option<bool> = parse_optional_bool,

        /// The debug adapter supports the 'setExpression' request.
        supportsSetExpression | "supportsSetExpression": Option<bool> = parse_optional_bool,

        /// The debug adapter supports the 'terminate' request.
        supportsTerminateRequest | "supportsTerminateRequest": Option<bool> = parse_optional_bool,

        /// The debug adapter supports data breakpoints.
        supportsDataBreakpoints | "supportsDataBreakpoints": Option<bool> = parse_optional_bool,

        /// The debug adapter supports the 'readMemory' request.
        supportsReadMemoryRequest | "supportsReadMemoryRequest": Option<bool> = parse_optional_bool,

        /// The debug adapter supports the 'writeMemory' request.
        supportsWriteMemoryRequest | "supportsWriteMemoryRequest": Option<bool> = parse_optional_bool,

        /// The debug adapter supports the 'disassemble' request.
        supportsDisassembleRequest | "supportsDisassembleRequest": Option<bool> = parse_optional_bool,

        /// The debug adapter supports the 'cancel' request.
        supportsCancelRequest | "supportsCancelRequest": Option<bool> = parse_optional_bool,

        /// The debug adapter supports the 'breakpointLocations' request.
        supportsBreakpointLocationsRequest | "supportsBreakpointLocationsRequest": Option<bool> = parse_optional_bool,

        /// The debug adapter supports the 'clipboard' context value in the 'evaluate'
        /// request.
        supportsClipboardContext | "supportsClipboardContext": Option<bool> = parse_optional_bool,

        /// The debug adapter supports stepping granularities (argument 'granularity')
        /// for the stepping requests.
        supportsSteppingGranularity | "supportsSteppingGranularity": Option<bool> = parse_optional_bool,

        /// The debug adapter supports adding breakpoints based on instruction
        /// references.
        supportsInstructionBreakpoints | "supportsInstructionBreakpoints": Option<bool> = parse_optional_bool,

        /// The debug adapter supports 'filterOptions' as an argument on the
        /// 'setExceptionBreakpoints' request.
        supportsExceptionFilterOptions | "supportsExceptionFilterOptions": Option<bool> = parse_optional_bool,

        /// The debug adapter supports the 'singleThread' property on the execution
        /// requests ('continue', 'next', 'stepIn', 'stepOut', 'reverseContinue',
        /// 'stepBack').
        supportsSingleThreadExecutionRequests | "supportsSingleThreadExecutionRequests": Option<bool> = parse_optional_bool,
    }
);
