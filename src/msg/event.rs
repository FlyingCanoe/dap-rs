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
mod continued_event;

pub use continued_event::ContinuedEvent;

#[derive(Clone, Debug)]
pub enum Event {
    Continue(ContinuedEvent),
}

impl Event {
    pub(crate) fn parse(msg: json::Value) -> anyhow::Result<Event> {
        let event_type = parse_string(msg.get("event"))?;

        let event = match event_type.as_str() {
            "continued" => Event::Continue(ContinuedEvent::parse(msg)?),
            _ => bail!("invalid event"),
        };
        Ok(event)
    }
}
