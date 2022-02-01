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
mod exited;
mod invalidated;
mod loaded_source;

pub use capabilities::CapabilitiesEvent;
pub use continued::ContinuedEvent;
pub use exited::ExitedEvent;
pub use invalidated::InvalidatedEvent;
pub use loaded_source::LoadedSourceEvent;

#[derive(Clone, Debug)]
pub enum Event {
    Capabilities(CapabilitiesEvent),
    Continue(ContinuedEvent),
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
}

impl ToValue for Event {
    fn to_value(self) -> Option<serde_json::Value> {
        match self {
            Event::Capabilities(event) => event.to_value(),
            Event::Continue(event) => event.to_value(),
            Event::Exited(event) => event.to_value(),
            Event::Initialized => {
                let mut msg = serde_json::Map::new();
                msg.insert("event".to_string(), "initialized".into());
                Some(msg.into())
            }
            Event::Invalidated(event) => event.to_value(),
            Event::LoadedSource(event) => event.to_value(),
        }
    }
}
