macro_rules! event {
    (
        $(#[$event_meta:meta])*
        $event_name:ident | $event:literal {
            $(
                $(#[$field_meta:meta])*
                $($field:ident).+ | $field_wire_name:literal: $field_ty:ty,
            )*
        }
    ) => {
        #[derive(Clone, Debug)]
        $(#[$event_meta])*
        pub struct $event_name {
            $(
                $(#[$field_meta])*
                pub $($field).+: $field_ty,
            )*
        }

        impl crate::utils::ToValue for $event_name {
            fn to_value(self) -> Option<serde_json::Value> {
                let mut msg = serde_json::Map::new();
                #[allow(unused_mut)]
                let mut body = serde_json::Map::new();

                msg.insert("event".to_string(), $event.into());

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

pub mod breakpoint;
pub mod capabilities;
pub mod continued;
pub mod exited;
pub mod invalidated;
pub mod loaded_source;
pub mod memory;
pub mod module;
pub mod output;
pub mod process;
pub mod progress_end;
pub mod progress_start;
pub mod progress_update;
pub mod stopped;
pub mod terminated;
pub mod thread;

use breakpoint::BreakpointEvent;
use capabilities::CapabilitiesEvent;
use continued::ContinuedEvent;
use exited::ExitedEvent;
use invalidated::InvalidatedEvent;
use loaded_source::LoadedSourceEvent;
use memory::MemoryEvent;
use module::ModuleEvent;
use output::OutputEvent;
use process::ProcessEvent;
use progress_end::ProgressEndEvent;
use progress_start::ProgressStartEvent;
use progress_update::ProgressUpdateEvent;
use stopped::StoppedEvent;
use terminated::TerminatedEvent;
use thread::ThreadEvent;

use crate::utils::ToValue;

#[derive(Clone, Debug)]
pub enum Event {
    Breakpoint(BreakpointEvent),
    Continue(ContinuedEvent),
    Capabilities(CapabilitiesEvent),
    Exited(ExitedEvent),
    /// This event indicates that the debug adapter is ready to accept configuration requests (e.g. SetBreakpointsRequest, SetExceptionBreakpointsRequest).
    /// A debug adapter is expected to send this event when it is ready to accept configuration requests (but not before the 'initialize' request has finished).
    /// The sequence of events/requests is as follows:
    /// - adapters sends 'initialized' event (after the 'initialize' request has returned)
    /// - frontend sends zero or more 'setBreakpoints' requests
    /// - frontend sends one 'setFunctionBreakpoints' request (if capability 'supportsFunctionBreakpoints' is true)
    /// - frontend sends a 'setExceptionBreakpoints' request if one or more 'exceptionBreakpointFilters' have been defined (or if 'supportsConfigurationDoneRequest' is not defined or false)
    /// - frontend sends other future configuration requests
    /// - frontend sends one 'configurationDone' request to indicate the end of the configuration.
    Initialized,
    Invalidated(InvalidatedEvent),
    LoadedSource(LoadedSourceEvent),
    MemoryEvent(MemoryEvent),
    Module(ModuleEvent),
    Output(OutputEvent),
    Process(ProcessEvent),
    ProgressEnd(ProgressEndEvent),
    ProgressStart(ProgressStartEvent),
    ProgressUpdate(ProgressUpdateEvent),
    Stopped(StoppedEvent),
    Terminated(TerminatedEvent),
    Thread(ThreadEvent),
}

impl ToValue for Event {
    fn to_value(self) -> Option<serde_json::Value> {
        match self {
            Event::Breakpoint(event) => event.to_value(),
            Event::Continue(event) => event.to_value(),
            Event::Capabilities(event) => event.to_value(),
            Event::Exited(event) => event.to_value(),
            Event::Initialized => {
                let mut msg = serde_json::Map::new();
                msg.insert("event".to_string(), "initialized".into());
                Some(msg.into())
            }
            Event::Invalidated(event) => event.to_value(),
            Event::LoadedSource(event) => event.to_value(),
            Event::MemoryEvent(event) => event.to_value(),
            Event::Module(event) => event.to_value(),
            Event::Output(event) => event.to_value(),
            Event::Process(event) => event.to_value(),
            Event::ProgressEnd(event) => event.to_value(),
            Event::ProgressStart(event) => event.to_value(),
            Event::ProgressUpdate(event) => event.to_value(),
            Event::Stopped(event) => event.to_value(),
            Event::Terminated(event) => event.to_value(),
            Event::Thread(event) => event.to_value(),
        }
    }
}
