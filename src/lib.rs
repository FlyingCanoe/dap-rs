macro_rules! request {
    (
        $(#[$request_meta:meta])*
        $request_name:ident | $command:literal {
            $(
                $(#[$field_meta:meta])*
                $field:ident | $field_wire_name:literal: $field_ty:ty,
            )*
        }
    ) => {
        use crate::adapter::Session;
        use crate::request::{Response, ErrorResponse, RequestExt};
        use crate::dap_type::Message;

        #[derive(Debug)]
        $(#[$request_meta])*
        pub struct $request_name {
            pub(crate) seq: u64,
            $(
                $(#[$field_meta])*
                pub $field: $field_ty,
            )*
        }

        impl $request_name {
            pub(crate) fn parse(msg: json::Value) -> anyhow::Result<$request_name> {
                use crate::utils::Parse;

                let _args = msg.get("arguments");
                let seq = u64::parse(msg.get("seq"))?;

                $(
                    let value = _args.ok_or(anyhow::Error::msg("invalid request"))?.get($field_wire_name);
                    let $field = <$field_ty as Parse>::parse(value)?;
                )*

                let request = $request_name {
                    seq,
                    $($field),*
                };
                Ok(request)
            }
        }

    };

}

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
                pub $field: $field_ty,
            )*
        }

        impl crate::utils::ToValue for $type_name {
            fn to_value(self) -> Option<json::Value> {
                let mut map = json::Map::new();
                $(
                    <$field_ty as crate::utils::ToValue>::to_value(self.$field)
                    .map(|value| {
                            map.insert(
                                $field_wire_name.to_string(),
                                value
                            )
                        }
                    );
                )*
                Some(map.into())
            }
        }
    };
}

macro_rules! dap_type_enum {
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
            Other(String),
            $(
                $(#[$field_meta])*
                $field,
            )*
        }

        impl crate::utils::Parse for $type_name {
            fn parse(input: Option<&json::Value>) -> anyhow::Result<$type_name> {
                let input = input
                    .ok_or(anyhow::Error::msg("parsing error"))?
                    .as_str()
                    .ok_or(anyhow::Error::msg("parsing error"))?;
                let output = match input {
                    $($field_wire_name => $type_name::$field,)*
                    _ => $type_name::Other(input.to_string()),
                };
                Ok(output)
            }
        }


        impl crate::utils::ToValue for $type_name {
            fn to_value(self) -> Option<json::Value> {
                let value = match self {
                    $(Self::$field => json::Value::String($field_wire_name.to_string()),)*
                    Self::Other(value) => json::Value::String(value),
                };
                Some(value)
            }
        }
    };

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
            fn parse(input: Option<&json::Value>) -> anyhow::Result<$type_name> {
                let input = String::parse(input)?;

                let output = match input.as_str() {
                    $($field_wire_name => $type_name::$field,)*
                    _ => anyhow::bail!("parsing error")
                };
                Ok(output)
            }
        }


        impl crate::utils::ToValue for $type_name {
            fn to_value(self) -> Option<json::Value> {
                let value = match self {
                    $(Self::$field => json::Value::String($field_wire_name.to_string()),)*
                };
                Some(value)
            }
        }
    };
}

pub mod adapter;
mod connection;
pub mod dap_type;
pub mod request;
mod utils;
