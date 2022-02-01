use anyhow::bail;
use serde_json as json;

use crate::utils::parse_string;

macro_rules! event {
    (
        $event_name:ident {
            $(
                $(#[$field_meta:meta])*
                $field:ident | $field_wire_name:literal: $field_ty:ty = $field_parsing_fn:expr,
            )*
        }
    ) => {
        #[derive(Clone, Debug)]
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
                    let $field = $field_parsing_fn(value)?;
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

pub use capabilities::CapabilitiesEvent;
pub use continued::ContinuedEvent;
pub use exited::ExitedEvent;
pub use loaded_source::LoadedSourceEvent;

#[derive(Clone, Debug)]
pub enum Event {
    Continue(ContinuedEvent),
    Capabilities(CapabilitiesEvent),
    Exited(ExitedEvent),
    LoadedSource(LoadedSourceEvent),
    Initialized,
}

impl Event {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<Event> {
        let event_type = parse_string(msg.get("event"))?;

        let event = match event_type.as_str() {
            "continued" => Event::Continue(ContinuedEvent::parse(msg)?),
            "capabilities" => Event::Capabilities(CapabilitiesEvent::parse(msg)?),
            "exited" => Event::Exited(ExitedEvent::parse(msg)?),
            "initialized" => Event::Initialized,
            _ => bail!("invalid event"),
        };
        Ok(event)
    }
}
