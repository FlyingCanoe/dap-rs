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
        use serde_json as json;


        #[derive(Debug, Clone)]
        pub struct $type_name {
            $(
                $(#[$field_meta])*
                $field: $field_ty,
            )*
        }

        impl $type_name {
            pub(crate) fn parse(input: Option<&json::Value>) -> anyhow::Result<$type_name> {
                let input = input.ok_or(Error::msg("parsing error"))?;
                $(
                    let value = input.get($field_wire_name);
                    let $field = $field_parsing_fn(value)?;
                )*

                let output = $type_name {
                    $(
                        $field,
                    )*
                };
                Ok(output)
            }

            pub(crate) fn parse_vec(
                input: Option<&json::Value>,
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
                input: Option<&json::Value>,
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

mod checksum;
mod checksum_algorithm;
mod data_breakpoint;
mod data_breakpoint_access_type;
mod evaluate_ctx;
mod exception_break_mode;
mod exception_filter_option;
mod exception_options;
mod exception_path_segment;
mod function_breakpoint;
mod instruction_breakpoint;
mod path_format;
mod presentation_hint;
mod source;
mod source_breakpoint;
mod stack_frame_format;
mod stepping_granularity;
mod value_format;
mod variables_filter;

pub use checksum::Checksum;
pub use checksum_algorithm::ChecksumAlgorithm;
pub use data_breakpoint::DataBreakpoint;
pub use data_breakpoint_access_type::DataBreakpointAccessType;
pub use evaluate_ctx::EvaluateCtx;
pub use exception_break_mode::ExceptionBreakMode;
pub use exception_filter_option::ExceptionFilterOptions;
pub use exception_options::ExceptionOptions;
pub use exception_path_segment::ExceptionPathSegment;
pub use function_breakpoint::FunctionBreakpoint;
pub use instruction_breakpoint::InstructionBreakpoint;
pub use path_format::PathFormat;
pub use presentation_hint::PresentationHint;
pub use source::Source;
pub use source_breakpoint::SourceBreakpoint;
pub use stack_frame_format::StackFrameFormat;
pub use stepping_granularity::SteppingGranularity;
pub use value_format::ValueFormat;
pub use variables_filter::VariablesFilter;
