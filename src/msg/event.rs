macro_rules! event {
    (
        $(#[$event_meta:meta])*
        $event_name:ident {
            $(
                $(#[$field_meta:meta])*
                $field:ident | $field_wire_name:literal: $field_ty:ty,
            )*
        }
    ) => {
        #[derive(Clone, Debug)]
        $(#[$event_meta])*
        pub struct $event_name {
            $(
                $(#[$field_meta])*
                $field: $field_ty,
            )*
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
