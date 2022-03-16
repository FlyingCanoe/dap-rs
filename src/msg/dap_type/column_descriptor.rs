dap_type_enum!(
    /// Datatype of values in this column.  Defaults to 'string' if not specified.
    Type {
        String | "string",
        Number | "number",
        Boolean | "boolean",
        UnixTimestampUTC | "unixTimestampUTC",
    }
);

dap_type_struct!(
    /// A ColumnDescriptor specifies what module attribute to show in a column of the ModulesView, how to format it,
    /// and what the column's label should be.
    /// It is only used if the underlying UI actually supports this level of customization.
    ColumnDescriptor {
        /// Name of the attribute rendered in this column.
        attribute_name | "attributeName": String,
        /// Format to use for the rendered values in this column. TBD how the format strings looks like.
        format | "format": Option<String>,
        /// Header UI label of column.
        label | "label": String,
        /// Width of this column in characters (hint only).
        width | "width": Option<u64>,
        /// Datatype of values in this column.  Defaults to 'string' if not specified.
        r#type | "type": Option<Type>,
    }
);
