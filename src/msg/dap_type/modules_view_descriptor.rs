use crate::msg::dap_type::column_descriptor::ColumnDescriptor;

dap_type_struct!(
    /// The ModulesViewDescriptor is the container for all declarative configuration options of a ModuleView.
    /// For now it only specifies the columns to be shown in the modules view.
    ModulesViewDescriptor {
        ///
        columns | "columns": Vec<ColumnDescriptor>,
    }
);
