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
                $($field).+: $field_ty,
            )*
        }

        impl crate::utils::ToValue for $event_name {
            fn to_value(self) -> serde_json::Value {
                let mut msg = serde_json::Map::new();
                #[allow(unused_mut)]
                let mut body = serde_json::Map::new();

                msg.insert("event".to_string(), $event.to_value());

                $(
                    body.insert($field_wire_name.to_string(), self.$($field).+.to_value());
                )*

                msg.insert("body".to_string(), serde_json::Value::Object(body));
                serde_json::Value::Object(msg)
            }
        }
    };
}

mod breakpoint;
mod capabilities;
mod continued;
mod exited;
mod initialized;
mod invalidated;
mod loaded_source;
mod memory;
mod module;
mod output;
mod process;
mod progress_end;
mod progress_start;
mod progress_update;
mod stopped;
mod terminated;
mod thread;

use breakpoint::BreakpointEvent;
use capabilities::CapabilitiesEvent;
use continued::ContinuedEvent;
use exited::ExitedEvent;
use initialized::InitializedEvent;
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
    Initialized(InitializedEvent),
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
    fn to_value(self) -> serde_json::Value {
        match self {
            Event::Breakpoint(event) => event.to_value(),
            Event::Continue(event) => event.to_value(),
            Event::Capabilities(event) => event.to_value(),
            Event::Exited(event) => event.to_value(),
            Event::Initialized(event) => event.to_value(),
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
