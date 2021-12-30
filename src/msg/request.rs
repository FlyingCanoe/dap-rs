use anyhow::{bail, Error};
use serde_json as json;

mod attach;
mod breakpoint_locations;
mod completions;
mod configuration_done;
mod continue_request;
mod data_breakpoint_info;
mod diassemble;
mod disconnect;
mod evaluate;
mod exception_info;
mod goto;
mod goto_targets;
mod initialize;
mod launch;
mod loaded_sources;
mod modules;
mod next;
mod pause;
mod read_memory;
mod restart;
mod restart_frame;
mod reverse_continue;
mod scopes;
mod set_breakpoint;
mod set_data_breakpoints;
mod set_exception_breakpoints;
mod set_expression;
mod set_function_breakpoints;
mod set_instruction_breakpoints;
mod set_variable;
mod source;
mod stacktrace;
mod step_back;
mod step_in;
mod step_in_targets;
mod step_out;
mod terminate;
mod terminate_threads;
mod threads;
mod variables;
mod write_memory;

use initialize::InitializeRequest;

use self::attach::AttachRequest;
use self::breakpoint_locations::BreakpointLocationRequest;
use self::completions::CompletionsRequest;
use self::configuration_done::ConfigurationDoneRequest;
use self::continue_request::ContinueRequest;
use self::data_breakpoint_info::DataBreakPointInfoRequest;
use self::diassemble::DiassambleRequest;
use self::disconnect::DisconnectRequest;
use self::evaluate::EvaluateRequest;
use self::exception_info::ExceptionInfoRequest;
use self::goto::GotoRequest;
use self::goto_targets::GotoTargetsRequest;
use self::launch::LaunchRequest;
use self::loaded_sources::LoadedSourcesRequest;
use self::modules::ModulesRequest;
use self::next::NextRequest;
use self::pause::PauseRequest;
use self::read_memory::ReadMemoryRequest;
use self::restart::RestartRequest;
use self::restart_frame::RestartFrameRequest;
use self::reverse_continue::ReverseContinueRequest;
use self::scopes::ScopesRequest;
use self::set_breakpoint::SetBreakpointsRequest;
use self::set_data_breakpoints::SetDataBreakpointRequest;
use self::set_exception_breakpoints::SetExceptionBreakpoints;
use self::set_expression::SetExpressionRequest;
use self::set_function_breakpoints::SetFunctionBreakpointRequest;
use self::set_instruction_breakpoints::SetInstructionBreakpointsRequest;
use self::set_variable::SetVariableRequest;
use self::source::SourceRequest;
use self::stacktrace::StackTraceRequest;
use self::step_back::StepBackRequest;
use self::step_in::StepInRequest;
use self::step_in_targets::StepInTargetRequest;
use self::step_out::StepOutRequest;
use self::terminate::TerminateRequest;
use self::terminate_threads::TerminateThreadsRequest;
use self::threads::ThreadsRequest;
use self::variables::VariablesRequest;
use self::write_memory::WriteMemoryRequest;

#[derive(Clone, Debug, Hash)]
pub enum Request {
    Initialize(InitializeRequest),
    ConfigurationDone(ConfigurationDoneRequest),
    Completions(CompletionsRequest),
    Launch(LaunchRequest),
    Attach(AttachRequest),
    Restart(RestartRequest),
    Disconnect(DisconnectRequest),
    Terminate(TerminateRequest),
    BreakpointLocations(BreakpointLocationRequest),
    SetBreakpoints(SetBreakpointsRequest),
    SetFunctionBreakpoints(SetFunctionBreakpointRequest),
    SetExceptionBreakpoints(SetExceptionBreakpoints),
    DataBreakpointInfo(DataBreakPointInfoRequest),
    SetDataBreakpoints(SetDataBreakpointRequest),
    SetInstructionBreakpoints(SetInstructionBreakpointsRequest),
    ContinueRequest(ContinueRequest),
    Next(NextRequest),
    StepIn(StepInRequest),
    StepOut(StepOutRequest),
    StepBack(StepBackRequest),
    ReverseContinue(ReverseContinueRequest),
    RestartFrame(RestartFrameRequest),
    Goto(GotoRequest),
    Pause(PauseRequest),
    StackTrace(StackTraceRequest),
    Scopes(ScopesRequest),
    Variables(VariablesRequest),
    SetVariable(SetVariableRequest),
    Source(SourceRequest),
    Threads(ThreadsRequest),
    TerminateThreads(TerminateThreadsRequest),
    Modules(ModulesRequest),
    LoadedSources(LoadedSourcesRequest),
    Evaluate(EvaluateRequest),
    SetExpression(SetExpressionRequest),
    StepInTargets(StepInTargetRequest),
    GotoTargets(GotoTargetsRequest),
    ExceptionInfo(ExceptionInfoRequest),
    ReadMemory(ReadMemoryRequest),
    WriteMemory(WriteMemoryRequest),
    Disassemble(DiassambleRequest),
}

impl Request {
    pub(crate) fn parse(mut msg: json::Map<String, json::Value>) -> anyhow::Result<Request> {
        use json::map::Entry;

        let request_type = match msg.entry("command") {
            Entry::Vacant(_) => bail!("invalid request"),
            Entry::Occupied(ref entry) => entry
                .get()
                .as_str()
                .ok_or(Error::msg("invalid request"))?
                .to_owned(),
        };

        let request = match request_type.as_str() {
            "initialize" => Request::Initialize(InitializeRequest::parse(msg)?),
            "configurationDone" => Request::ConfigurationDone(
                configuration_done::ConfigurationDoneRequest::parse(msg)?,
            ),
            "completions" => Request::Completions(completions::CompletionsRequest::parse(msg)?),
            "launch" => Request::Launch(launch::LaunchRequest::parse(msg)?),
            "attach" => Request::Attach(attach::AttachRequest::parse(msg)?),
            "restart" => Request::Restart(restart::RestartRequest::parse(msg)?),
            "disconnect" => Request::Disconnect(disconnect::DisconnectRequest::parse(msg)?),
            "terminate" => Request::Terminate(terminate::TerminateRequest::parse(msg)?),
            "breakpointLocations" => Request::BreakpointLocations(
                breakpoint_locations::BreakpointLocationRequest::parse(msg)?,
            ),
            "setBreakpoints" => {
                Request::SetBreakpoints(set_breakpoint::SetBreakpointsRequest::parse(msg)?)
            }
            "setFunctionBreakpoints" => Request::SetFunctionBreakpoints(
                set_function_breakpoints::SetFunctionBreakpointRequest::parse(msg)?,
            ),
            "setExceptionBreakpoints" => Request::SetExceptionBreakpoints(
                set_exception_breakpoints::SetExceptionBreakpoints::parse(msg)?,
            ),
            "dataBreakpointInfo" => Request::DataBreakpointInfo(
                data_breakpoint_info::DataBreakPointInfoRequest::parse(msg)?,
            ),
            "setDataBreakpoints" => Request::SetDataBreakpoints(
                set_data_breakpoints::SetDataBreakpointRequest::parse(msg)?,
            ),
            "setInstructionBreakpoints" => Request::SetInstructionBreakpoints(
                set_instruction_breakpoints::SetInstructionBreakpointsRequest::parse(msg)?,
            ),

            "continue" => Request::ContinueRequest(continue_request::ContinueRequest::parse(msg)?),
            "next" => Request::Next(next::NextRequest::parse(msg)?),
            "stepIn" => Request::StepIn(step_in::StepInRequest::parse(msg)?),
            "stepOut" => Request::StepOut(step_out::StepOutRequest::parse(msg)?),
            "stepBack" => Request::StepBack(step_back::StepBackRequest::parse(msg)?),
            "reverseContinue" => {
                Request::ReverseContinue(reverse_continue::ReverseContinueRequest::parse(msg)?)
            }
            "restartFrame" => {
                Request::RestartFrame(restart_frame::RestartFrameRequest::parse(msg)?)
            }
            "goto" => Request::Goto(goto::GotoRequest::parse(msg)?),
            "pause" => Request::Pause(pause::PauseRequest::parse(msg)?),
            "stackTrace" => Request::StackTrace(stacktrace::StackTraceRequest::parse(msg)?),
            "scopes" => Request::Scopes(scopes::ScopesRequest::parse(msg)?),
            "variables" => Request::Variables(variables::VariablesRequest::parse(msg)?),
            "setVariable" => Request::SetVariable(set_variable::SetVariableRequest::parse(msg)?),
            "source" => Request::Source(source::SourceRequest::parse(msg)?),
            "threads" => Request::Threads(threads::ThreadsRequest::parse(msg)?),
            "terminateThreads" => {
                Request::TerminateThreads(terminate_threads::TerminateThreadsRequest::parse(msg)?)
            }
            "modules" => Request::Modules(modules::ModulesRequest::parse(msg)?),
            "loadedSources" => {
                Request::LoadedSources(loaded_sources::LoadedSourcesRequest::parse(msg)?)
            }
            "evaluate" => Request::Evaluate(evaluate::EvaluateRequest::parse(msg)?),
            "setExpression" => {
                Request::SetExpression(set_expression::SetExpressionRequest::parse(msg)?)
            }
            "stepInTargets" => {
                Request::StepInTargets(step_in_targets::StepInTargetRequest::parse(msg)?)
            }
            "gotoTargets" => Request::GotoTargets(goto_targets::GotoTargetsRequest::parse(msg)?),
            "exceptionInfo" => {
                Request::ExceptionInfo(exception_info::ExceptionInfoRequest::parse(msg)?)
            }
            "readMemory" => Request::ReadMemory(read_memory::ReadMemoryRequest::parse(msg)?),
            "writeMemory" => Request::WriteMemory(write_memory::WriteMemoryRequest::parse(msg)?),
            "disassemble" => Request::Disassemble(diassemble::DiassambleRequest::parse(msg)?),
            _ => bail!("invalid request"),
        };
        Ok(request)
    }
}
