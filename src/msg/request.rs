use anyhow::bail;
use serde_json as json;

use crate::utils::{Parse, ToValue};

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
        #[derive(Clone, Debug)]
        $(#[$request_meta])*
        pub struct $request_name {
            $(
                $(#[$field_meta])*
                $($field)+: $field_ty,
            )*
        }

        impl $request_name {
            pub(crate) fn parse(msg: serde_json::Value) -> anyhow::Result<$request_name> {
                let _args = msg.get("arguments").ok_or(anyhow::Error::msg("invalid request"))?;

                $(
                    let value = _args.get($field_wire_name);
                    let $($field)+ = <$field_ty as crate::utils::Parse>::parse(value)?;
                )*

                let request = $request_name {
                    $($($field)+),*

                };
                Ok(request)
            }
        }

        impl crate::utils::ToValue for $request_name {
            fn to_value(self) -> serde_json::Value {
                let mut msg = serde_json::Map::new();
                #[allow(unused_mut)]
                let mut arguments = serde_json::Map::new();

                msg.insert(
                    "type".to_string(),
                    serde_json::Value::String("response".to_string()),
                );

                msg.insert("command".to_string(), $command.to_value());

                $(
                    arguments.insert($field_wire_name.to_string(), <$field_ty as crate::utils::ToValue>::to_value(self.$($field).+));
                )*

                msg.insert("arguments".to_string(), serde_json::Value::Object(arguments));
                serde_json::Value::Object(msg)
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
                $($field).+: $field_ty,
            )*
        }

        impl crate::utils::ToValue for $response_name {
            fn to_value(self) -> serde_json::Value {
                let mut msg = serde_json::Map::new();
                #[allow(unused_mut)]
                let mut body = serde_json::Map::new();

                msg.insert(
                    "type".to_string(),
                    serde_json::Value::String("response".to_string()),
                );

                $(
                    body.insert($field_wire_name.to_string(), <$field_ty as crate::utils::ToValue>::to_value(self.$($field).+));
                )*

                msg.insert("body".to_string(), serde_json::Value::Object(body));
                serde_json::Value::Object(msg)
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

pub use attach::{AttachRequest, AttachResponse};
pub use breakpoint_locations::{BreakpointLocationsRequest, BreakpointLocationsResponse};
pub use completions::{CompletionsRequest, CompletionsResponse};
pub use configuration_done::{ConfigurationDoneRequest, ConfigurationDoneResponse};
pub use continue_request::{ContinueRequest, ContinueResponse};
pub use data_breakpoint_info::{DataBreakpointInfoRequest, DataBreakpointInfoResponse};
pub use disassemble::{DisassembleRequest, DisassembleResponse};
pub use disconnect::{DisconnectRequest, DisconnectResponse};
pub use evaluate::{EvaluateRequest, EvaluateResponse};
pub use exception_info::{ExceptionInfoRequest, ExceptionInfoResponse};
pub use goto::{GotoRequest, GotoResponse};
pub use goto_targets::{GotoTargetsRequest, GotoTargetsResponse};
pub use initialize::{InitializeRequest, InitializeResponse};
pub use launch::{LaunchRequest, LaunchResponse};
pub use loaded_sources::{LoadedSourcesRequest, LoadedSourcesResponse};
pub use modules::{ModulesRequest, ModulesResponse};
pub use next::{NextRequest, NextResponse};
pub use pause::{PauseRequest, PauseResponse};
pub use read_memory::{ReadMemoryRequest, ReadMemoryResponse};
pub use restart::{RestartRequest, RestartResponse};
pub use restart_frame::{RestartFrameRequest, RestartFrameResponse};
pub use reverse_continue::{ReverseContinueRequest, ReverseContinueResponse};
pub use scopes::{ScopesRequest, ScopesResponse};
pub use set_breakpoints::{SetBreakpointsRequest, SetBreakpointsResponse};
pub use set_data_breakpoints::{SetDataBreakpointsRequest, SetDataBreakpointsResponse};
pub use set_exception_breakpoints::{
    SetExceptionBreakpointsRequest, SetExceptionBreakpointsResponse,
};
pub use set_expression::{SetExpressionRequest, SetExpressionResponse};
pub use set_function_breakpoints::{SetFunctionBreakpointsRequest, SetFunctionBreakpointsResponse};
pub use set_instruction_breakpoints::{
    SetInstructionBreakpointsRequest, SetInstructionBreakpointsResponse,
};
pub use set_variable::{SetVariableRequest, SetVariableResponse};
pub use source::{SourceRequest, SourceResponse};
pub use stack_trace::{StackTraceRequest, StackTraceResponse};
pub use step_back::{StepBackRequest, StepBackResponse};
pub use step_in::{StepInRequest, StepInResponse};
pub use step_in_targets::{StepInTargetsRequest, StepInTargetsResponse};
pub use step_out::{StepOutRequest, StepOutResponse};
pub use terminate::{TerminateRequest, TerminateResponse};
pub use terminate_threads::{TerminateThreadsRequest, TerminateThreadsResponse};
pub use threads::{ThreadsRequest, ThreadsResponse};
pub use variables::{VariablesRequest, VariablesResponse};
pub use write_memory::{WriteMemoryRequest, WriteMemoryResponse};

#[derive(Clone, Debug)]
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

impl ToValue for Request {
    fn to_value(self) -> json::Value {
        match self {
            Request::Initialize(request) => request.to_value(),
            Request::ConfigurationDone(request) => request.to_value(),
            Request::Completions(request) => request.to_value(),
            Request::Launch(request) => request.to_value(),
            Request::Attach(request) => request.to_value(),
            Request::Restart(request) => request.to_value(),
            Request::Disconnect(request) => request.to_value(),
            Request::Terminate(request) => request.to_value(),
            Request::BreakpointLocations(request) => request.to_value(),
            Request::SetBreakpoints(request) => request.to_value(),
            Request::SetFunctionBreakpoints(request) => request.to_value(),
            Request::SetExceptionBreakpoints(request) => request.to_value(),
            Request::DataBreakpointInfo(request) => request.to_value(),
            Request::SetDataBreakpoints(request) => request.to_value(),
            Request::SetInstructionBreakpoints(request) => request.to_value(),
            Request::ContinueRequest(request) => request.to_value(),
            Request::Next(request) => request.to_value(),
            Request::StepIn(request) => request.to_value(),
            Request::StepOut(request) => request.to_value(),
            Request::StepBack(request) => request.to_value(),
            Request::ReverseContinue(request) => request.to_value(),
            Request::RestartFrame(request) => request.to_value(),
            Request::Goto(request) => request.to_value(),
            Request::Pause(request) => request.to_value(),
            Request::StackTrace(request) => request.to_value(),
            Request::Scopes(request) => request.to_value(),
            Request::Variables(request) => request.to_value(),
            Request::SetVariable(request) => request.to_value(),
            Request::Source(request) => request.to_value(),
            Request::Continue(request) => request.to_value(),
            Request::Threads(request) => request.to_value(),
            Request::TerminateThreads(request) => request.to_value(),
            Request::Modules(request) => request.to_value(),
            Request::LoadedSources(request) => request.to_value(),
            Request::Evaluate(request) => request.to_value(),
            Request::SetExpression(request) => request.to_value(),
            Request::StepInTargets(request) => request.to_value(),
            Request::GotoTargets(request) => request.to_value(),
            Request::ExceptionInfo(request) => request.to_value(),
            Request::ReadMemory(request) => request.to_value(),
            Request::WriteMemory(request) => request.to_value(),
            Request::Disassemble(request) => request.to_value(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Response {
    pub(crate) request_seq: u64,
    pub(crate) response_type: ResponseType,
}

#[derive(Clone, Debug)]
pub enum ResponseType {
    Initialize(InitializeResponse),
    ConfigurationDone(ConfigurationDoneResponse),
    Completions(CompletionsResponse),
    Launch(LaunchResponse),
    Attach(AttachResponse),
    Restart(RestartResponse),
    Disconnect(DisconnectResponse),
    Terminate(TerminateResponse),
    BreakpointLocations(BreakpointLocationsResponse),
    SetBreakpoints(SetBreakpointsResponse),
    SetFunctionBreakpoints(SetFunctionBreakpointsResponse),
    SetExceptionBreakpoints(SetExceptionBreakpointsResponse),
    DataBreakpointInfo(DataBreakpointInfoResponse),
    SetDataBreakpoints(SetDataBreakpointsResponse),
    SetInstructionBreakpoints(SetInstructionBreakpointsResponse),
    ContinueResponse(ContinueResponse),
    Next(NextResponse),
    StepIn(StepInResponse),
    StepOut(StepOutResponse),
    StepBack(StepBackResponse),
    ReverseContinue(ReverseContinueResponse),
    RestartFrame(RestartFrameResponse),
    Goto(GotoResponse),
    Pause(PauseResponse),
    StackTrace(StackTraceResponse),
    Scopes(ScopesResponse),
    Variables(VariablesResponse),
    SetVariable(SetVariableResponse),
    Source(SourceResponse),
    Continue(ContinueResponse),
    Threads(ThreadsResponse),
    TerminateThreads(TerminateThreadsResponse),
    Modules(ModulesResponse),
    LoadedSources(LoadedSourcesResponse),
    Evaluate(EvaluateResponse),
    SetExpression(SetExpressionResponse),
    StepInTargets(StepInTargetsResponse),
    GotoTargets(GotoTargetsResponse),
    ExceptionInfo(ExceptionInfoResponse),
    ReadMemory(ReadMemoryResponse),
    WriteMemory(WriteMemoryResponse),
    Disassemble(DisassembleResponse),
}

impl ToValue for Response {
    fn to_value(self) -> json::Value {
        let mut value = match self.response_type {
            ResponseType::Initialize(response) => response.to_value(),
            ResponseType::ConfigurationDone(response) => response.to_value(),
            ResponseType::Completions(response) => response.to_value(),
            ResponseType::Launch(response) => response.to_value(),
            ResponseType::Attach(response) => response.to_value(),
            ResponseType::Restart(response) => response.to_value(),
            ResponseType::Disconnect(response) => response.to_value(),
            ResponseType::Terminate(response) => response.to_value(),
            ResponseType::BreakpointLocations(response) => response.to_value(),
            ResponseType::SetBreakpoints(response) => response.to_value(),
            ResponseType::SetFunctionBreakpoints(response) => response.to_value(),
            ResponseType::SetExceptionBreakpoints(response) => response.to_value(),
            ResponseType::DataBreakpointInfo(response) => response.to_value(),
            ResponseType::SetDataBreakpoints(response) => response.to_value(),
            ResponseType::SetInstructionBreakpoints(response) => response.to_value(),
            ResponseType::ContinueResponse(response) => response.to_value(),
            ResponseType::Next(response) => response.to_value(),
            ResponseType::StepIn(response) => response.to_value(),
            ResponseType::StepOut(response) => response.to_value(),
            ResponseType::StepBack(response) => response.to_value(),
            ResponseType::ReverseContinue(response) => response.to_value(),
            ResponseType::RestartFrame(response) => response.to_value(),
            ResponseType::Goto(response) => response.to_value(),
            ResponseType::Pause(response) => response.to_value(),
            ResponseType::StackTrace(response) => response.to_value(),
            ResponseType::Scopes(response) => response.to_value(),
            ResponseType::Variables(response) => response.to_value(),
            ResponseType::SetVariable(response) => response.to_value(),
            ResponseType::Source(response) => response.to_value(),
            ResponseType::Continue(response) => response.to_value(),
            ResponseType::Threads(response) => response.to_value(),
            ResponseType::TerminateThreads(response) => response.to_value(),
            ResponseType::Modules(response) => response.to_value(),
            ResponseType::LoadedSources(response) => response.to_value(),
            ResponseType::Evaluate(response) => response.to_value(),
            ResponseType::SetExpression(response) => response.to_value(),
            ResponseType::StepInTargets(response) => response.to_value(),
            ResponseType::GotoTargets(response) => response.to_value(),
            ResponseType::ExceptionInfo(response) => response.to_value(),
            ResponseType::ReadMemory(response) => response.to_value(),
            ResponseType::WriteMemory(response) => response.to_value(),
            ResponseType::Disassemble(response) => response.to_value(),
        };

        let map = value.as_object_mut().unwrap();
        map.insert("request_seq".to_string(), self.request_seq.into());

        value
    }
}
