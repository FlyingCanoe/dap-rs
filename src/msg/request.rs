use anyhow::{bail, Context};
use serde_json as json;

use crate::utils::get_str;

macro_rules! request {
    (
        $(#[$request_meta:meta])*
        $request_name:ident {
            $(
                $(#[$field_meta:meta])*
                $field:ident | $field_wire_name:literal: $field_ty:ty,
            )*
        }
    ) => {
        #[derive(Clone, Debug)]
        $(#[$request_meta])*
        pub struct $request_name {
            $(
                $(#[$field_meta])*
                $field: $field_ty,
            )*
        }

        impl $request_name {
            pub(crate) fn parse(msg: serde_json::Value) -> anyhow::Result<$request_name> {
                use crate::utils::Parse;
                let args = msg.get("arguments").ok_or(anyhow::Error::msg("invalid request"))?;

                $(
                    let value = msg.get($field_wire_name);
                    let $field = <$field_ty>::parse(value)?;
                )*

                let request = $request_name {
                    $(
                        $field,
                    )*

                };
                Ok(request)
            }
        }
    };
}

mod attach;
mod breakpoint_locations;
mod completions;
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
mod variables;
mod write_memory;

use initialize::InitializeRequest;

use self::attach::AttachRequest;
use self::breakpoint_locations::BreakpointLocationsRequest;
use self::completions::CompletionsRequest;
use self::continue_request::ContinueRequest;
use self::data_breakpoint_info::DataBreakpointInfoRequest;
use self::diassemble::DiassambleRequest;
use self::disconnect::DisconnectRequest;
use self::evaluate::EvaluateRequest;
use self::exception_info::ExceptionInfoRequest;
use self::goto::GotoRequest;
use self::goto_targets::GotoTargetsRequest;
use self::launch::LaunchRequest;
use self::modules::ModulesRequest;
use self::next::NextRequest;
use self::pause::PauseRequest;
use self::read_memory::ReadMemoryRequest;
use self::restart::RestartRequest;
use self::restart_frame::RestartFrameRequest;
use self::reverse_continue::ReverseContinueRequest;
use self::scopes::ScopesRequest;
use self::set_breakpoint::SetBreakpointsRequest;
use self::set_data_breakpoints::SetDataBreakpointsRequest;
use self::set_exception_breakpoints::SetExceptionBreakpointsRequest;
use self::set_expression::SetExpressionRequest;
use self::set_function_breakpoints::SetFunctionBreakpointsRequest;
use self::set_instruction_breakpoints::SetInstructionBreakpointsRequest;
use self::set_variable::SetVariableRequest;
use self::source::SourceRequest;
use self::stacktrace::StackTraceRequest;
use self::step_back::StepBackRequest;
use self::step_in::StepInRequest;
use self::step_in_targets::StepInTargetsRequest;
use self::step_out::StepOutRequest;
use self::terminate::TerminateRequest;
use self::terminate_threads::TerminateThreadsRequest;
use self::variables::VariablesRequest;
use self::write_memory::WriteMemoryRequest;

#[derive(Clone, Debug)]
pub enum Request {
    Initialize(InitializeRequest),
    ConfigurationDone,
    Completions(CompletionsRequest),
    Launch(LaunchRequest),
    Attach(AttachRequest),
    Restart(RestartRequest),
    Disconnect(DisconnectRequest),
    Terminate(TerminateRequest),
    BreakpointLocations(BreakpointLocationsRequest),
    SetBreakpoints(SetBreakpointsRequest),
    SetFunctionBreakpoints(SetFunctionBreakpointsRequest),
    SetExceptionBreakpoints(SetExceptionBreakpointsRequest),
    DataBreakpointInfo(DataBreakpointInfoRequest),
    SetDataBreakpoints(SetDataBreakpointsRequest),
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
    Threads,
    TerminateThreads(TerminateThreadsRequest),
    Modules(ModulesRequest),
    LoadedSources,
    Evaluate(EvaluateRequest),
    SetExpression(SetExpressionRequest),
    StepInTargets(StepInTargetsRequest),
    GotoTargets(GotoTargetsRequest),
    ExceptionInfo(ExceptionInfoRequest),
    ReadMemory(ReadMemoryRequest),
    WriteMemory(WriteMemoryRequest),
    Disassemble(DiassambleRequest),
}

impl Request {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<Request> {
        let request_type = get_str(&msg, "command").context("invalid request")?;

        let request = match request_type {
            "initialize" => Request::Initialize(InitializeRequest::parse(msg)?),
            "configurationDone" => Request::ConfigurationDone,
            "completions" => Request::Completions(completions::CompletionsRequest::parse(msg)?),
            "launch" => Request::Launch(launch::LaunchRequest::parse(msg)?),
            "attach" => Request::Attach(attach::AttachRequest::parse(msg)?),
            "restart" => Request::Restart(restart::RestartRequest::parse(msg)?),
            "disconnect" => Request::Disconnect(disconnect::DisconnectRequest::parse(msg)?),
            "terminate" => Request::Terminate(terminate::TerminateRequest::parse(msg)?),
            "breakpointLocations" => Request::BreakpointLocations(
                breakpoint_locations::BreakpointLocationsRequest::parse(msg)?,
            ),
            "setBreakpoints" => {
                Request::SetBreakpoints(set_breakpoint::SetBreakpointsRequest::parse(msg)?)
            }
            "setFunctionBreakpoints" => Request::SetFunctionBreakpoints(
                set_function_breakpoints::SetFunctionBreakpointsRequest::parse(msg)?,
            ),
            "setExceptionBreakpoints" => Request::SetExceptionBreakpoints(
                set_exception_breakpoints::SetExceptionBreakpointsRequest::parse(msg)?,
            ),
            "dataBreakpointInfo" => Request::DataBreakpointInfo(
                data_breakpoint_info::DataBreakpointInfoRequest::parse(msg)?,
            ),
            "setDataBreakpoints" => Request::SetDataBreakpoints(
                set_data_breakpoints::SetDataBreakpointsRequest::parse(msg)?,
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
            "threads" => Request::Threads,
            "terminateThreads" => {
                Request::TerminateThreads(terminate_threads::TerminateThreadsRequest::parse(msg)?)
            }
            "modules" => Request::Modules(modules::ModulesRequest::parse(msg)?),
            "loadedSources" => Request::LoadedSources,
            "evaluate" => Request::Evaluate(evaluate::EvaluateRequest::parse(msg)?),
            "setExpression" => {
                Request::SetExpression(set_expression::SetExpressionRequest::parse(msg)?)
            }
            "stepInTargets" => {
                Request::StepInTargets(step_in_targets::StepInTargetsRequest::parse(msg)?)
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
