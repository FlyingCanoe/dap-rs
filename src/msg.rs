use anyhow::bail;
use serde_json as json;

use crate::utils::Parse;

macro_rules! dap_type_struct {
    (
        $(#[$type_meta:meta])*
        $type_name:ident {
            $(
                $(#[$field_meta:meta])*
                $field:ident | $field_wire_name:literal: $field_ty:ty,
            )*
        }
    ) => {
        #[derive(Debug, Clone)]
        $(#[$type_meta])*
        pub struct $type_name {
            $(
                $(#[$field_meta])*
                $field: $field_ty,
            )*
        }

        impl crate::utils::Parse for $type_name {
            fn parse(input: Option<&serde_json::Value>) -> anyhow::Result<$type_name> {
                let input = input.ok_or(anyhow::Error::msg("parsing error"))?;
                $(
                    let value = input.get($field_wire_name);
                    let $field = <$field_ty>::parse(value)?;
                )*

                let output = $type_name {
                    $(
                        $field,
                    )*
                };
                Ok(output)
            }
        }

        impl $type_name {

        }
    };
}

macro_rules! dap_type_enum {
    (
        $(#[$type_meta:meta])*
        $type_name:ident {
            $(
                $(#[$field_meta:meta])*
                $field:ident | $field_wire_name:literal,
            )*
        }
    ) => {
        #[derive(Clone, Debug)]
        $(#[$type_meta])*
        pub enum $type_name {
            $(
                $(#[$field_meta])*
                $field,
            )*
        }

        impl crate::utils::Parse for $type_name {
            fn parse(input: Option<&serde_json::Value>) -> anyhow::Result<$type_name> {
                let input = input
                    .ok_or(anyhow::Error::msg("parsing error"))?
                    .as_str()
                    .ok_or(anyhow::Error::msg("parsing error"))?;
                let output = match input {
                    $($field_wire_name => $type_name::$field,)*
                    _ => anyhow::bail!("parsing error"),
                };
                Ok(output)
            }
        }
    };

    (
        $(#[$type_meta:meta])*
        $type_name:ident {
            Other,
            $(
                $(#[$field_meta:meta])*
                $field:ident | $field_wire_name:literal,
            )*
        }
    ) => {
        #[derive(Clone, Debug)]
        $(#[$type_meta])*
        pub enum $type_name {
            Other,
            $(
                $(#[$field_meta])*
                $field,
            )*
        }

        impl crate::utils::Parse for $type_name {
            fn parse(input: Option<&serde_json::Value>) -> anyhow::Result<$type_name> {
                let input = input
                    .ok_or(anyhow::Error::msg("parsing error"))?
                    .as_str()
                    .ok_or(anyhow::Error::msg("parsing error"))?;
                let output = match input {
                    $($field_wire_name => $type_name::$field,)*
                    _ => $type_name::Other,
                };
                Ok(output)
            }
        }
    };
}

pub mod dap_type;
pub mod event;
pub mod request;

use event::Event;
use request::{Request, Response};

#[derive(Clone, Debug)]
pub struct Msg {
    seq: u64,
    msg_type: MsgType,
}

#[derive(Clone, Debug)]
pub enum MsgType {
    Request(Request),
    Response(Response),
    Event(Event),
}

pub(crate) fn parse_msg(raw_msg: &str) -> anyhow::Result<Msg> {
    let msg: json::Value = json::from_str(raw_msg)?;

    let seq = u64::parse(msg.get("seq"))?;
    let msg_type = String::parse(msg.get("type"))?;

    let msg_type = match msg_type.as_str() {
        "request" => MsgType::Request(Request::parse(msg)?),
        _ => {
            bail!("invalid msg")
        }
    };

    let msg = Msg { seq, msg_type };
    Ok(msg)
}
