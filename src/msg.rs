macro_rules! dap_type_struct {
    (
        $(#[$type_meta:meta])*
        $type_name:ident {
            $(
                $(#[$field_meta:meta])*
                $($field:ident).+ | $field_wire_name:literal: $field_ty:ty,
            )*
        }
    ) => {
        #[derive(Debug, Clone)]
        $(#[$type_meta])*
        pub struct $type_name {
            $(
                $(#[$field_meta])*
                pub(crate) $($field)+: $field_ty,
            )*
        }

        impl crate::utils::Parse for $type_name {
            fn parse(input: Option<&serde_json::Value>) -> anyhow::Result<$type_name> {
                let input = input.ok_or(anyhow::Error::msg("parsing error"))?;
                $(
                    let value = input.get($field_wire_name);
                    let $($field)+ = <$field_ty>::parse(value)?;
                )*

                let output = $type_name {
                    $(
                        $($field)+,
                    )*
                };
                Ok(output)
            }
        }

        impl crate::utils::ToValue for $type_name {
            fn to_value(self) -> Option<serde_json::Value> {
                let mut map = serde_json::Map::new();
                $(
                    <$field_ty as crate::utils::ToValue>::to_value(self.$($field).+)
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
            fn parse(input: Option<&serde_json::Value>) -> anyhow::Result<$type_name> {
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
            fn to_value(self) -> Option<serde_json::Value> {
                let value = match self {
                    $(Self::$field => serde_json::Value::String($field_wire_name.to_string()),)*
                    Self::Other(value) =>serde_json::Value::String(value),
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

        impl crate::utils::ToValue for $type_name {
            fn to_value(self) -> Option<serde_json::Value> {
                let value = match self {
                    $(Self::$field => serde_json::Value::String($field_wire_name.to_string())),*
                };
                Some(value)
            }
        }
    };
}

pub mod dap_type;
pub mod event;
pub mod request;
