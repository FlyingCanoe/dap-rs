use crate::utils::{parse_optional_string, parse_optional_u64, parse_string};

dap_type_struct!(
    ColumnDescriptor {
        /// Name of the attribute rendered in this column.
        attribute_name | "attributeName": String = parse_string,

        /// Header UI label of column.
        label | "label": String = parse_string,

        /// Format to use for the rendered values in this column. TBD how the format
        /// Strings looks like.
        format | "format": Option<String> = parse_optional_string,

        /// Datatype of values in this column.  Defaults to 'String' if not specified.
        /// Values: 'String', 'u64', 'bool', 'unixTimestampUTC', etc.
        datatype | "type": Option<ColumnDescriptorType> = ColumnDescriptorType::parse_optional,

        /// Width of this column in characters (hint only).
        width | "width": Option<u64> = parse_optional_u64,
    }
);

dap_type_enum!(
    ColumnDescriptorType {
        String | "string",
        U64 | "number",
        Bool | "boolean",
        UnixTimestampUtc | "unixTimestampUTC",
    }
);
