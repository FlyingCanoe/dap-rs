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

        impl $event_name {
            pub(crate) fn parse(msg: serde_json::Value) -> anyhow::Result<$event_name> {
                let args = msg.get("body").ok_or(anyhow::Error::msg("invalid event"))?;

                $(
                    let value = msg.get($field_wire_name);
                    let $field = <$field_ty as crate::utils::Parse>::parse(value)?;
                )*

                let event = $event_name {
                    $(
                        $field,
                    )*

                };
                Ok(event)
            }
        }
    };
}

mod capabilities;
mod continued;
mod exited;
mod invalidated;
mod loaded_source;
mod memory_event;

pub use capabilities::CapabilitiesEvent;
pub use continued::ContinuedEvent;
pub use exited::ExitedEvent;
pub use invalidated::InvalidatedEvent;
pub use loaded_source::LoadedSourceEvent;
pub use memory_event::MemoryEvent;

#[derive(Clone, Debug)]
pub enum Event {
    Continue(ContinuedEvent),
    Capabilities(CapabilitiesEvent),
    Exited(ExitedEvent),
    Initialized,
    Invalidated(InvalidatedEvent),
    LoadedSource(LoadedSourceEvent),
    MemoryEvent(MemoryEvent),
}
