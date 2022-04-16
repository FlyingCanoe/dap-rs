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
        #[derive(Debug)]
        $(#[$request_meta])*
        pub struct $request_name {
            $(
                $(#[$field_meta])*
                pub $field: $field_ty,
            )*
        }

        impl $request_name {
            pub(crate) fn parse(msg: json::Value) -> anyhow::Result<$request_name> {
                let _args = msg.get("arguments");

                $(
                    let value = _args.ok_or(anyhow::Error::msg("invalid request"))?.get($field_wire_name);
                    let $field = <$field_ty as crate::utils::Parse>::parse(value)?;
                )*

                let request = $request_name {
                    $($field),*
                };
                Ok(request)
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
    };
}

pub mod adapter;
mod connection;
pub mod request;
mod utils;
