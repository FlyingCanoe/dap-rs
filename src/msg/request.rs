use anyhow::bail;
use serde_json as json;

use crate::codec::Session;
use crate::utils::{Parse, ToValue};

macro_rules!  request {
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
                let _args = msg.get("arguments").ok_or(anyhow::Error::msg("invalid request"))?;
                let seq = msg.get("seq")
                    .ok_or(anyhow::Error::msg("invalid msg"))?
                    .as_u64()
                    .ok_or(anyhow::Error::msg("invalid msg"))?;

                $(
                    let value = _args.get($field_wire_name);
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
                let _args = msg.get("arguments").ok_or(anyhow::Error::msg("invalid request"))?;
                let seq = msg.get("seq")
                    .ok_or(anyhow::Error::msg("invalid msg"))?
                    .as_u64()
                    .ok_or(anyhow::Error::msg("invalid msg"))?;

                $(
                    let value = _args.get($field_wire_name);
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
                use crate::msg::request::ResponseType;
                use crate::msg::request::{Response, AcknowledgementResponse};

                let response_type = match response {
                    Ok(_) => ResponseType::from(AcknowledgementResponse::new("attach".to_string())),
                    Err(err) => ResponseType::from(err),
                };

                let seq = session.next_seq();
                session.connection.send_response(Response {
                    seq,
                    request_seq: self.seq,
                    response_type,
                })?;
                Ok(())
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
                let _args = msg.get("arguments").ok_or(anyhow::Error::msg("invalid request"))?;
                let seq = msg.get("seq")
                    .ok_or(anyhow::Error::msg("invalid msg"))?
                    .as_u64()
                    .ok_or(anyhow::Error::msg("invalid msg"))?;

                $(
                    let value = _args.get($field_wire_name);
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
            type Response = $response;

            fn respond(
                self,
                response: Result<$response, crate::msg::request::ErrorResponse>,
                session: &mut crate::msg::request::Session,
            ) -> Result<(), anyhow::Error> {
                use crate::msg::request::ResponseType;
                use crate::msg::request::Response;

                let response_type = match response {
                    Ok(response) => ResponseType::from(response),
                    Err(err) => ResponseType::from(err),
                };

                let seq = session.next_seq();
                session.connection.send_response(Response {
                    seq,
                    request_seq: self.seq,
                    response_type,
                })?;
                Ok(())
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

mod acknowledgement;
mod attach;
mod breakpoint_locations;
mod completions;
mod configuration_done;
mod continue_request;
mod data_breakpoint_info;
mod disassemble;
mod disconnect;
mod error;
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

pub(crate) use self::acknowledgement::AcknowledgementResponse;
pub use attach::AttachRequest;
pub use breakpoint_locations::{BreakpointLocationsRequest, BreakpointLocationsResponse};
pub use completions::{CompletionsRequest, CompletionsResponse};
pub use configuration_done::ConfigurationDoneRequest;
pub use continue_request::{ContinueRequest, ContinueResponse};
pub use data_breakpoint_info::{DataBreakpointInfoRequest, DataBreakpointInfoResponse};
pub use disassemble::{DisassembleRequest, DisassembleResponse};
pub use disconnect::DisconnectRequest;
pub use error::ErrorResponse;
pub use evaluate::{EvaluateRequest, EvaluateResponse};
pub use exception_info::{ExceptionInfoRequest, ExceptionInfoResponse};
pub use goto::GotoRequest;
pub use goto_targets::{GotoTargetsRequest, GotoTargetsResponse};
pub use initialize::{InitializeRequest, InitializeResponse};
pub use launch::LaunchRequest;
pub use loaded_sources::{LoadedSourcesRequest, LoadedSourcesResponse};
pub use modules::{ModulesRequest, ModulesResponse};
pub use next::NextRequest;
pub use pause::PauseRequest;
pub use read_memory::{ReadMemoryRequest, ReadMemoryResponse};
pub use restart::RestartRequest;
pub use restart_frame::RestartFrameRequest;
pub use reverse_continue::ReverseContinueRequest;
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
pub use step_back::StepBackRequest;
pub use step_in::StepInRequest;
pub use step_in_targets::{StepInTargetsRequest, StepInTargetsResponse};
pub use step_out::StepOutRequest;
pub use terminate::TerminateRequest;
pub use terminate_threads::TerminateThreadsRequest;
pub use threads::{ThreadsRequest, ThreadsResponse};
pub use variables::{VariablesRequest, VariablesResponse};
pub use write_memory::{WriteMemoryRequest, WriteMemoryResponse};

pub trait RequestExt {
    type Response;

    fn respond(
        self,
        response: Result<Self::Response, ErrorResponse>,
        session: &mut Session,
    ) -> Result<(), anyhow::Error>;
}

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
    pub(crate) fn parse(msg: &str) -> anyhow::Result<Request> {
        let request: json::Value = json::from_str(msg)?;
        let request_type = String::parse(request.get("command"))?;

        let request = match request_type.as_str() {
            "attach" => Request::Attach(AttachRequest::parse(request)?),
            "breakpointLocations" => {
                Request::BreakpointLocations(BreakpointLocationsRequest::parse(request)?)
            }
            "completions" => Request::Completions(CompletionsRequest::parse(request)?),
            "configurationDone" => {
                Request::ConfigurationDone(ConfigurationDoneRequest::parse(request)?)
            }
            "continue" => Request::Continue(ContinueRequest::parse(request)?),
            "dataBreakpointInfo" => {
                Request::DataBreakpointInfo(DataBreakpointInfoRequest::parse(request)?)
            }
            "disassemble" => Request::Disassemble(DisassembleRequest::parse(request)?),
            "disconnect" => Request::Disconnect(DisconnectRequest::parse(request)?),
            "evaluate" => Request::Evaluate(EvaluateRequest::parse(request)?),
            "exceptionInfo" => Request::ExceptionInfo(ExceptionInfoRequest::parse(request)?),
            "goto" => Request::Goto(GotoRequest::parse(request)?),
            "gotoTargets" => Request::GotoTargets(GotoTargetsRequest::parse(request)?),
            "initialize" => Request::Initialize(InitializeRequest::parse(request)?),
            "launch" => Request::Launch(LaunchRequest::parse(request)?),
            "loadedSources" => Request::LoadedSources(LoadedSourcesRequest::parse(request)?),
            "modules" => Request::Modules(ModulesRequest::parse(request)?),
            "next" => Request::Next(NextRequest::parse(request)?),
            "pause" => Request::Pause(PauseRequest::parse(request)?),
            "readMemory" => Request::ReadMemory(ReadMemoryRequest::parse(request)?),
            "restartFrame" => Request::RestartFrame(RestartFrameRequest::parse(request)?),
            "restart" => Request::Restart(RestartRequest::parse(request)?),
            "reverseContinue" => Request::ReverseContinue(ReverseContinueRequest::parse(request)?),
            "scopes" => Request::Scopes(ScopesRequest::parse(request)?),
            "setBreakpoints" => Request::SetBreakpoints(SetBreakpointsRequest::parse(request)?),
            "setDataBreakpoints" => {
                Request::SetDataBreakpoints(SetDataBreakpointsRequest::parse(request)?)
            }
            "setExceptionBreakpoints" => {
                Request::SetExceptionBreakpoints(SetExceptionBreakpointsRequest::parse(request)?)
            }
            "setExpression" => Request::SetExpression(SetExpressionRequest::parse(request)?),
            "setFunctionBreakpoints" => {
                Request::SetFunctionBreakpoints(SetFunctionBreakpointsRequest::parse(request)?)
            }
            "setInstructionBreakpoints" => Request::SetInstructionBreakpoints(
                SetInstructionBreakpointsRequest::parse(request)?,
            ),
            "setVariable" => Request::SetVariable(SetVariableRequest::parse(request)?),
            "source" => Request::Source(SourceRequest::parse(request)?),
            "stackTrace" => Request::StackTrace(StackTraceRequest::parse(request)?),
            "stepBack" => Request::StepBack(StepBackRequest::parse(request)?),
            "stepIn" => Request::StepIn(StepInRequest::parse(request)?),
            "stepInTargets" => Request::StepInTargets(StepInTargetsRequest::parse(request)?),
            "stepOut" => Request::StepOut(StepOutRequest::parse(request)?),
            "terminate" => Request::Terminate(TerminateRequest::parse(request)?),
            "terminateThreads" => {
                Request::TerminateThreads(TerminateThreadsRequest::parse(request)?)
            }
            "threads" => Request::Threads(ThreadsRequest::parse(request)?),
            "variables" => Request::Variables(VariablesRequest::parse(request)?),
            "writeMemory" => Request::WriteMemory(WriteMemoryRequest::parse(request)?),

            _ => bail!("invalid request"),
        };
        Ok(request)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Response {
    pub(crate) seq: u64,
    pub(crate) request_seq: u64,
    pub(crate) response_type: ResponseType,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub(crate) enum ResponseType {
    Acknowledgement(AcknowledgementResponse),
    Error(ErrorResponse),
    Initialize(InitializeResponse),
    Completions(CompletionsResponse),
    BreakpointLocations(BreakpointLocationsResponse),
    SetBreakpoints(SetBreakpointsResponse),
    SetFunctionBreakpoints(SetFunctionBreakpointsResponse),
    SetExceptionBreakpoints(SetExceptionBreakpointsResponse),
    DataBreakpointInfo(DataBreakpointInfoResponse),
    SetDataBreakpoints(SetDataBreakpointsResponse),
    SetInstructionBreakpoints(SetInstructionBreakpointsResponse),
    ContinueResponse(ContinueResponse),
    StackTrace(StackTraceResponse),
    Scopes(ScopesResponse),
    Variables(VariablesResponse),
    SetVariable(SetVariableResponse),
    Source(SourceResponse),
    Continue(ContinueResponse),
    Threads(ThreadsResponse),
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
    fn to_value(self) -> Option<json::Value> {
        let mut value = match self.response_type {
            ResponseType::Initialize(response) => response.to_value(),
            ResponseType::Completions(response) => response.to_value(),
            ResponseType::BreakpointLocations(response) => response.to_value(),
            ResponseType::SetBreakpoints(response) => response.to_value(),
            ResponseType::SetFunctionBreakpoints(response) => response.to_value(),
            ResponseType::SetExceptionBreakpoints(response) => response.to_value(),
            ResponseType::DataBreakpointInfo(response) => response.to_value(),
            ResponseType::SetDataBreakpoints(response) => response.to_value(),
            ResponseType::SetInstructionBreakpoints(response) => response.to_value(),
            ResponseType::ContinueResponse(response) => response.to_value(),
            ResponseType::StackTrace(response) => response.to_value(),
            ResponseType::Scopes(response) => response.to_value(),
            ResponseType::Variables(response) => response.to_value(),
            ResponseType::SetVariable(response) => response.to_value(),
            ResponseType::Source(response) => response.to_value(),
            ResponseType::Continue(response) => response.to_value(),
            ResponseType::Threads(response) => response.to_value(),
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
            ResponseType::Acknowledgement(response) => response.to_value(),
            ResponseType::Error(error) => error.to_value(),
        }?;

        let map = value.as_object_mut().unwrap();
        map.insert("request_seq".to_string(), self.request_seq.into());
        map.insert("seq".to_string(), self.seq.into());

        Some(value)
    }
}

impl From<ModulesResponse> for ResponseType {
    fn from(v: ModulesResponse) -> Self {
        Self::Modules(v)
    }
}

impl From<DisassembleResponse> for ResponseType {
    fn from(v: DisassembleResponse) -> Self {
        Self::Disassemble(v)
    }
}

impl From<WriteMemoryResponse> for ResponseType {
    fn from(v: WriteMemoryResponse) -> Self {
        Self::WriteMemory(v)
    }
}

impl From<ReadMemoryResponse> for ResponseType {
    fn from(v: ReadMemoryResponse) -> Self {
        Self::ReadMemory(v)
    }
}

impl From<ExceptionInfoResponse> for ResponseType {
    fn from(v: ExceptionInfoResponse) -> Self {
        Self::ExceptionInfo(v)
    }
}

impl From<GotoTargetsResponse> for ResponseType {
    fn from(v: GotoTargetsResponse) -> Self {
        Self::GotoTargets(v)
    }
}

impl From<StepInTargetsResponse> for ResponseType {
    fn from(v: StepInTargetsResponse) -> Self {
        Self::StepInTargets(v)
    }
}

impl From<SetExpressionResponse> for ResponseType {
    fn from(v: SetExpressionResponse) -> Self {
        Self::SetExpression(v)
    }
}

impl From<EvaluateResponse> for ResponseType {
    fn from(v: EvaluateResponse) -> Self {
        Self::Evaluate(v)
    }
}

impl From<LoadedSourcesResponse> for ResponseType {
    fn from(v: LoadedSourcesResponse) -> Self {
        Self::LoadedSources(v)
    }
}

impl From<ThreadsResponse> for ResponseType {
    fn from(v: ThreadsResponse) -> Self {
        Self::Threads(v)
    }
}

impl From<SourceResponse> for ResponseType {
    fn from(v: SourceResponse) -> Self {
        Self::Source(v)
    }
}

impl From<SetVariableResponse> for ResponseType {
    fn from(v: SetVariableResponse) -> Self {
        Self::SetVariable(v)
    }
}

impl From<VariablesResponse> for ResponseType {
    fn from(v: VariablesResponse) -> Self {
        Self::Variables(v)
    }
}

impl From<ScopesResponse> for ResponseType {
    fn from(v: ScopesResponse) -> Self {
        Self::Scopes(v)
    }
}

impl From<StackTraceResponse> for ResponseType {
    fn from(v: StackTraceResponse) -> Self {
        Self::StackTrace(v)
    }
}

impl From<ContinueResponse> for ResponseType {
    fn from(v: ContinueResponse) -> Self {
        Self::ContinueResponse(v)
    }
}

impl From<SetInstructionBreakpointsResponse> for ResponseType {
    fn from(v: SetInstructionBreakpointsResponse) -> Self {
        Self::SetInstructionBreakpoints(v)
    }
}

impl From<SetDataBreakpointsResponse> for ResponseType {
    fn from(v: SetDataBreakpointsResponse) -> Self {
        Self::SetDataBreakpoints(v)
    }
}

impl From<DataBreakpointInfoResponse> for ResponseType {
    fn from(v: DataBreakpointInfoResponse) -> Self {
        Self::DataBreakpointInfo(v)
    }
}

impl From<SetExceptionBreakpointsResponse> for ResponseType {
    fn from(v: SetExceptionBreakpointsResponse) -> Self {
        Self::SetExceptionBreakpoints(v)
    }
}

impl From<SetFunctionBreakpointsResponse> for ResponseType {
    fn from(v: SetFunctionBreakpointsResponse) -> Self {
        Self::SetFunctionBreakpoints(v)
    }
}

impl From<SetBreakpointsResponse> for ResponseType {
    fn from(v: SetBreakpointsResponse) -> Self {
        Self::SetBreakpoints(v)
    }
}

impl From<BreakpointLocationsResponse> for ResponseType {
    fn from(v: BreakpointLocationsResponse) -> Self {
        Self::BreakpointLocations(v)
    }
}

impl From<CompletionsResponse> for ResponseType {
    fn from(v: CompletionsResponse) -> Self {
        Self::Completions(v)
    }
}

impl From<InitializeResponse> for ResponseType {
    fn from(v: InitializeResponse) -> Self {
        Self::Initialize(v)
    }
}

impl From<ErrorResponse> for ResponseType {
    fn from(v: ErrorResponse) -> Self {
        Self::Error(v)
    }
}

impl From<AcknowledgementResponse> for ResponseType {
    fn from(v: AcknowledgementResponse) -> Self {
        Self::Acknowledgement(v)
    }
}
