dap_type_struct!(
    ColumnDescriptor {
        /// Name of the attribute rendered in this column.
        attribute_name | "attributeName": String,

        /// Header UI label of column.
        label | "label": String,

        /// Format to use for the rendered values in this column. TBD how the format
        /// Strings looks like.
        format | "format": Option<String>,

        /// Datatype of values in this column.  Defaults to 'String' if not specified.
        /// Values: 'String', 'u64', 'bool', 'unixTimestampUTC', etc.
        datatype | "type": Option<ColumnDescriptorType> ,

        /// Width of this column in characters (hint only).
        width | "width": Option<u64>,
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
