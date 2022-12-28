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

                msg.insert("body".to_string(), serde_json::Value::Object(body));
                Some(msg.into())
            }
        }
    };
}

pub mod breakpoint;
pub mod capabilities;
pub mod continued;
pub mod exited;
pub mod initialized;
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

pub use breakpoint::BreakpointEvent;
pub use capabilities::CapabilitiesEvent;
pub use continued::ContinuedEvent;
pub use exited::ExitedEvent;
pub use initialized::InitializedEvent;
pub use invalidated::InvalidatedEvent;
pub use loaded_source::LoadedSourceEvent;
pub use memory::MemoryEvent;
pub use module::ModuleEvent;
pub use output::OutputEvent;
pub use process::ProcessEvent;
pub use progress_end::ProgressEndEvent;
pub use progress_start::ProgressStartEvent;
pub use progress_update::ProgressUpdateEvent;
pub use stopped::StoppedEvent;
pub use terminated::TerminatedEvent;
pub use thread::ThreadEvent;

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
    fn to_value(self) -> Option<serde_json::Value> {
        let mut value = match self {
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
        };
        value
            .as_mut()?
            .as_object_mut()?
            .insert("type".to_string(), "event".into());
        value
    }
}
