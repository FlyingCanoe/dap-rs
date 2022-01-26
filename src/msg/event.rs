use crate::utils::ToValue;

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

mod capabilities;
mod continued;

pub use capabilities::CapabilitiesEvent;
pub use continued::ContinuedEvent;

#[derive(Clone, Debug)]
pub enum Event {
    Capabilities(CapabilitiesEvent),
    Continue(ContinuedEvent),
}

impl ToValue for Event {
    fn to_value(self) -> Option<serde_json::Value> {
        match self {
            Event::Capabilities(event) => event.to_value(),
            Event::Continue(event) => event.to_value(),
        }
    }
}
