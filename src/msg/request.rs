use anyhow::{bail, Error};
use serde_json as json;

use crate::utils::Parse;

macro_rules! request {
    (
        $(#[$request_meta:meta])*
        $request_name:ident | $command:literal {
            $(
                $(#[$field_meta:meta])*
                $($field:ident).+ | $field_wire_name:literal: $field_ty:ty,
            )*
        }
    ) => {
        #[derive(Debug)]
        $(#[$request_meta])*
        pub struct $request_name {
            pub(crate) seq: u64,
            $(
                $(#[$field_meta])*
                pub $($field)+: $field_ty,
            )*
        }

        impl $request_name {
            pub(crate) fn parse(msg: serde_json::Value) -> anyhow::Result<$request_name> {
                let _args = msg.get("arguments");
                let seq = msg.get("seq")
                    .ok_or(anyhow::Error::msg("invalid msg"))?
                    .as_u64()
                    .ok_or(anyhow::Error::msg("invalid msg"))?;

                $(
                    let value = _args.ok_or(anyhow::Error::msg("invalid request"))?.get($field_wire_name);
                    let $($field)+ = <$field_ty as crate::utils::Parse>::parse(value)?;
                )*

                let request = $request_name {
                    seq,
                    $($($field)+),*
                };
                Ok(request)
            }
        }
    };
    (
        type Response = ();
        $(#[$request_meta:meta])*
        $request_name:ident | $command:literal {
            $(
                $(#[$field_meta:meta])*
                $($field:ident).+ | $field_wire_name:literal: $field_ty:ty,
            )*
        }
    ) => {
        #[derive(Debug)]
        $(#[$request_meta])*
        pub struct $request_name {
            pub(crate) seq: u64,
            $(
                $(#[$field_meta])*
                pub $($field)+: $field_ty,
            )*
        }

        impl $request_name {
            pub(crate) fn parse(msg: serde_json::Value) -> anyhow::Result<$request_name> {
                let _args = msg.get("arguments");
                let seq = msg.get("seq")
                    .ok_or(anyhow::Error::msg("invalid msg"))?
                    .as_u64()
                    .ok_or(anyhow::Error::msg("invalid msg"))?;

                $(
                    let value = _args.ok_or(anyhow::Error::msg("invalid request"))?.get($field_wire_name);
                    let $($field)+ = <$field_ty as crate::utils::Parse>::parse(value)?;
                )*

                let request = $request_name {
                    seq,
                    $($($field)+),*
                };
                Ok(request)
            }
        }

        impl crate::msg::request::RequestExt for $request_name {
            type Response = ();

            fn respond(
                self,
                response: Result<(), crate::msg::request::ErrorResponse>,
                session: &mut crate::msg::request::Session,
            ) -> Result<(), anyhow::Error> {
                use crate::msg::request::{Response, AcknowledgementResponse};

                let response = match response {
                    Ok(_) => Response::from(AcknowledgementResponse::new($command.to_string())),
                    Err(err) => Response::from(err),
                };

                session.send_response(response, self.seq)
            }
        }
    };
    (
        type Response = $response:ty;
        $(#[$request_meta:meta])*
        $request_name:ident | $command:literal {
            $(
                $(#[$field_meta:meta])*
                $($field:ident).+ | $field_wire_name:literal: $field_ty:ty,
            )*
        }
    ) => {
        #[derive(Debug)]
        $(#[$request_meta])*
        pub struct $request_name {
            pub(crate) seq: u64,
            $(
                $(#[$field_meta])*
                pub $($field)+: $field_ty,
            )*
        }

        impl $request_name {
            pub(crate) fn parse(msg: serde_json::Value) -> anyhow::Result<$request_name> {
                let _args = msg.get("arguments");
                let seq = msg.get("seq")
                    .ok_or(anyhow::Error::msg("invalid msg"))?
                    .as_u64()
                    .ok_or(anyhow::Error::msg("invalid msg"))?;

                $(
                    let value = _args.ok_or(anyhow::Error::msg("invalid request"))?.get($field_wire_name);
                    let $($field)+ = <$field_ty as crate::utils::Parse>::parse(value)?;
                )*

                let request = $request_name {
                    seq,
                    $($($field)+),*
                };
                Ok(request)
            }
        }
    };

}

macro_rules! response {
    (
        $(#[$request_meta:meta])*
        $response_name:ident | $command:literal {
            $(
                $(#[$field_meta:meta])*
                $($field:ident).+ | $field_wire_name:literal: $field_ty:ty,
            )*
        }
    ) => {
        #[derive(Clone, Debug)]
        $(#[$request_meta])*
        pub struct $response_name {
            $(
                $(#[$field_meta])*
                pub $($field).+: $field_ty,
            )*
        }

        impl crate::utils::ToValue for $response_name {
            fn to_value(self) -> Option<serde_json::Value> {
                #[allow(unused_mut)]
                let mut body = serde_json::Map::new();
                let mut msg = serde_json::Map::new();

                msg.insert("type".to_string(), "response".into());
                msg.insert("success".to_string(), true.into());
                msg.insert("command".to_string(), $command.into());

                $(
                    <$field_ty as crate::utils::ToValue>::to_value(self.$($field).+)
                    .map(|value| {
                            body.insert(
                                $field_wire_name.to_string(),
                                value
                            )
                        }
                    );
                )*

                msg.insert("body".to_string(), body.into());
                Some(msg.into())
            }
        }
    };
}

mod attach;
mod breakpoint_locations;
mod completions;
mod configuration_done;
mod continue_request;
mod data_breakpoint_info;
mod disassemble;
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
mod set_breakpoints;
mod set_data_breakpoints;
mod set_exception_breakpoints;
mod set_expression;
mod set_function_breakpoints;
mod set_instruction_breakpoints;
mod set_variable;
mod source;
mod stack_trace;
mod step_back;
mod step_in;
mod step_in_targets;
mod step_out;
mod terminate;
mod terminate_threads;
mod threads;
mod variables;
mod write_memory;

use attach::AttachRequest;
use breakpoint_locations::BreakpointLocationsRequest;
use completions::CompletionsRequest;
use configuration_done::ConfigurationDoneRequest;
use continue_request::ContinueRequest;
use data_breakpoint_info::DataBreakpointInfoRequest;
use disassemble::DisassembleRequest;
use disconnect::DisconnectRequest;
use evaluate::EvaluateRequest;
use exception_info::ExceptionInfoRequest;
use goto::GotoRequest;
use goto_targets::GotoTargetsRequest;
use initialize::InitializeRequest;
use launch::LaunchRequest;
use loaded_sources::LoadedSourcesRequest;
use modules::ModulesRequest;
use next::NextRequest;
use pause::PauseRequest;
use read_memory::ReadMemoryRequest;
use restart::RestartRequest;
use restart_frame::RestartFrameRequest;
use reverse_continue::ReverseContinueRequest;
use scopes::ScopesRequest;
use set_breakpoints::SetBreakpointsRequest;
use set_data_breakpoints::SetDataBreakpointsRequest;
use set_exception_breakpoints::SetExceptionBreakpointsRequest;
use set_expression::SetExpressionRequest;
use set_function_breakpoints::SetFunctionBreakpointsRequest;
use set_instruction_breakpoints::SetInstructionBreakpointsRequest;
use set_variable::SetVariableRequest;
use source::SourceRequest;
use stack_trace::StackTraceRequest;
use step_back::StepBackRequest;
use step_in::StepInRequest;
use step_in_targets::StepInTargetsRequest;
use step_out::StepOutRequest;
use terminate::TerminateRequest;
use terminate_threads::TerminateThreadsRequest;
use threads::ThreadsRequest;
use variables::VariablesRequest;
use write_memory::WriteMemoryRequest;

#[derive(Debug)]
pub enum Request {
    Initialize(InitializeRequest),
    ConfigurationDone(ConfigurationDoneRequest),
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
    Continue(ContinueRequest),
    Threads(ThreadsRequest),
    TerminateThreads(TerminateThreadsRequest),
    Modules(ModulesRequest),
    LoadedSources(LoadedSourcesRequest),
    Evaluate(EvaluateRequest),
    SetExpression(SetExpressionRequest),
    StepInTargets(StepInTargetsRequest),
    GotoTargets(GotoTargetsRequest),
    ExceptionInfo(ExceptionInfoRequest),
    ReadMemory(ReadMemoryRequest),
    WriteMemory(WriteMemoryRequest),
    Disassemble(DisassembleRequest),
}

impl Request {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<Request> {
        let request_type = String::parse(msg.get("command"))?;

        let request = match request_type.as_str() {
            "attach" => Request::Attach(AttachRequest::parse(msg)?),
            "breakpointLocations" => {
                Request::BreakpointLocations(BreakpointLocationsRequest::parse(msg)?)
            }
            "completions" => Request::Completions(CompletionsRequest::parse(msg)?),
            "configurationDone" => {
                Request::ConfigurationDone(ConfigurationDoneRequest::parse(msg)?)
            }
            "continue" => Request::Continue(ContinueRequest::parse(msg)?),
            "dataBreakpointInfo" => {
                Request::DataBreakpointInfo(DataBreakpointInfoRequest::parse(msg)?)
            }
            "disassemble" => Request::Disassemble(DisassembleRequest::parse(msg)?),
            "disconnect" => Request::Disconnect(DisconnectRequest::parse(msg)?),
            "evaluate" => Request::Evaluate(EvaluateRequest::parse(msg)?),
            "exceptionInfo" => Request::ExceptionInfo(ExceptionInfoRequest::parse(msg)?),
            "goto" => Request::Goto(GotoRequest::parse(msg)?),
            "gotoTargets" => Request::GotoTargets(GotoTargetsRequest::parse(msg)?),
            "initialize" => Request::Initialize(InitializeRequest::parse(msg)?),
            "launch" => Request::Launch(LaunchRequest::parse(msg)?),
            "loadedSources" => Request::LoadedSources(LoadedSourcesRequest::parse(msg)?),
            "modules" => Request::Modules(ModulesRequest::parse(msg)?),
            "next" => Request::Next(NextRequest::parse(msg)?),
            "pause" => Request::Pause(PauseRequest::parse(msg)?),
            "readMemory" => Request::ReadMemory(ReadMemoryRequest::parse(msg)?),
            "restartFrame" => Request::RestartFrame(RestartFrameRequest::parse(msg)?),
            "restart" => Request::Restart(RestartRequest::parse(msg)?),
            "reverseContinue" => Request::ReverseContinue(ReverseContinueRequest::parse(msg)?),
            "scopes" => Request::Scopes(ScopesRequest::parse(msg)?),
            "setBreakpoints" => Request::SetBreakpoints(SetBreakpointsRequest::parse(msg)?),
            "setDataBreakpoints" => {
                Request::SetDataBreakpoints(SetDataBreakpointsRequest::parse(msg)?)
            }
            "setExceptionBreakpoints" => {
                Request::SetExceptionBreakpoints(SetExceptionBreakpointsRequest::parse(msg)?)
            }
            "setExpression" => Request::SetExpression(SetExpressionRequest::parse(msg)?),
            "setFunctionBreakpoints" => {
                Request::SetFunctionBreakpoints(SetFunctionBreakpointsRequest::parse(msg)?)
            }
            "setInstructionBreakpoints" => {
                Request::SetInstructionBreakpoints(SetInstructionBreakpointsRequest::parse(msg)?)
            }
            "setVariable" => Request::SetVariable(SetVariableRequest::parse(msg)?),
            "source" => Request::Source(SourceRequest::parse(msg)?),
            "stackTrace" => Request::StackTrace(StackTraceRequest::parse(msg)?),
            "stepBack" => Request::StepBack(StepBackRequest::parse(msg)?),
            "stepIn" => Request::StepIn(StepInRequest::parse(msg)?),
            "stepInTargets" => Request::StepInTargets(StepInTargetsRequest::parse(msg)?),
            "stepOut" => Request::StepOut(StepOutRequest::parse(msg)?),
            "terminate" => Request::Terminate(TerminateRequest::parse(msg)?),
            "terminateThreads" => Request::TerminateThreads(TerminateThreadsRequest::parse(msg)?),
            "threads" => Request::Threads(ThreadsRequest::parse(msg)?),
            "variables" => Request::Variables(VariablesRequest::parse(msg)?),
            "writeMemory" => Request::WriteMemory(WriteMemoryRequest::parse(msg)?),

            _ => bail!("invalid request"),
        };
        Ok(request)
    }
}

#[derive(Debug)]
pub struct Response {}
