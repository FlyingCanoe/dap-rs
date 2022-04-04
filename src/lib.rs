macro_rules! builder {
    (
        type BuildedType = $builded_type:ident;
        $(#[$builder_meta:meta])*
        $builder:ident {
            #[doc=r" optional field"]
            $($optional_field:ident: Option $optional_field_ty:ty,)*
            #[doc=r" non optional field"]
            $($field:ident: $field_ty:ty,)*
        }
    ) => {
        $(#[$builder_meta])*
        pub struct $builder {
            $($optional_field: Option<$optional_field_ty>,)*
            $($field: $field_ty),*
        }

        impl $builder {
            pub fn new($($field: $field_ty),*) -> Self {
                Self {
                    $($field,)*
                    $($optional_field: None),*
                }
            }

            $(
                pub fn $field(mut self, $field: $field_ty) -> Self {
                    self.$field = $field;
                    self
                }
            )*

            $(
                pub fn $optional_field(mut self, $optional_field: $optional_field_ty) -> Self {
                    self.$optional_field = Some($optional_field);
                    self
                }
            )*

            pub fn build(self) -> $builded_type {
                $builded_type {
                    $($optional_field: self.$optional_field,)*
                    $($field: self.$field,)*
                }
            }
        }

    };
}

pub mod codec;
mod connection;
pub mod msg;
mod utils;
