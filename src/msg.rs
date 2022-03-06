use anyhow::{bail, Context};
use serde_json as json;

use crate::utils::{get_str, get_u64};

macro_rules! dap_type_struct {
    (
        $type_name:ident {
            $(
                $(#[$field_meta:meta])*
                $field:ident | $field_wire_name:literal: $field_ty:ty = $field_parsing_fn:expr,
            )*
        }
    ) => {
        use anyhow::Error;
        use fallible_iterator::{convert, FallibleIterator};


        #[derive(Debug, Clone)]
        pub struct $type_name {
            $(
                $(#[$field_meta])*
                $field: $field_ty,
            )*
        }

        impl crate::utils::Parse for $type_name {
            fn parse(input: Option<&serde_json::Value>) -> anyhow::Result<$type_name> {
                let input = input.ok_or(Error::msg("parsing error"))?;
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
            pub(crate) fn parse(input: Option<&serde_json::Value>) -> anyhow::Result<$type_name> {
                use crate::utils::Parse;
                let input = input.ok_or(Error::msg("parsing error"))?;
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

            pub(crate) fn parse_optional(input: Option<&serde_json::Value>) -> anyhow::Result<Option<$type_name>> {
                if input.is_some() {
                    let output = $type_name::parse(input)?;
                    Ok(Some(output))
                } else
                    {Ok(None)
                }
            }

            pub(crate) fn parse_vec(
                input: Option<&serde_json::Value>,
            ) -> anyhow::Result<Vec<$type_name>> {
                let input = input.ok_or(Error::msg("parsing error"))?;
                let iter = input
                    .as_array()
                    .ok_or(Error::msg("parsing error"))?
                    .iter()
                    .map(|value| $type_name::parse(Some(value)));
                let output: Vec<_> = convert(iter).collect()?;
                Ok(output)
            }

            pub(crate) fn parse_optional_vec(
                input: Option<&serde_json::Value>,
            ) -> anyhow::Result<Option<Vec<$type_name>>> {
                if input.is_some() {
                    let output = $type_name::parse_vec(input)?;
                    Ok(Some(output))
                } else {
                    Ok(None)
                }
            }
        }
    };
}

macro_rules! dap_type_enum {
    (
        $type_name:ident {
            $(
                $(#[$field_meta:meta])*
                $field:ident | $field_wire_name:literal,
            )*
        }
    ) => {
        #[derive(Clone, Debug)]
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

        impl $type_name {
            pub(crate) fn parse(input: Option<&serde_json::Value>) -> anyhow::Result<$type_name> {
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

            pub(crate) fn parse_optional(
                input: Option<&serde_json::Value>,
            ) -> anyhow::Result<Option<$type_name>> {
                if input.is_some() {
                    let output = $type_name::parse(input)?;
                    Ok(Some(output))
                } else {
                    Ok(None)
                }
            }

            pub(crate) fn parse_vec(
                input: Option<&serde_json::Value>,
            ) -> anyhow::Result<Vec<$type_name>> {
                use fallible_iterator::FallibleIterator;

                let iter = input.ok_or(anyhow::Error::msg("parsing error"))?.as_array().ok_or(anyhow::Error::msg("parsing error"))?.iter().map(|value| $type_name::parse(Some(value)));
                let output: Vec<_> = fallible_iterator::convert(iter).collect()?;
                Ok(output)
            }

            pub(crate) fn parse_optional_vec(
                input: Option<&serde_json::Value>,
            ) -> anyhow::Result<Option<Vec<$type_name>>> {
                if input.is_some() {
                    let output = $type_name::parse_vec(input)?;
                    Ok(Some(output))
                } else {
                    Ok(None)
                }
            }
        }
    };
}

pub mod dap_type;
pub mod event;
pub mod request;
pub mod response;

use event::Event;
use request::Request;
use response::Response;

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

    let seq = get_u64(&msg, "seq").context("invalid_msg")?;
    let msg_type = get_str(&msg, "type").context("invalid msg")?;

    let msg_type = match msg_type {
        "request" => MsgType::Request(Request::parse(msg)?),
        "response" => MsgType::Response(Response::parse(msg)?),
        "event" => MsgType::Event(Event::parse(msg)?),
        _ => {
            bail!("invalid msg")
        }
    };

    let msg = Msg { seq, msg_type };
    Ok(msg)
}
