use either::Either;

dap_type_struct!(
    /// A Module object represents a row in the modules view.
    /// Two attributes are mandatory: an id identifies a module in the modules view and is used in a ModuleEvent for identifying a module for adding, updating or deleting.
    /// The name is used to minimally render the module in the UI.
    /// 
    /// Additional attributes can be added to the module. They will show up in the module View if they have a corresponding ColumnDescriptor.
    /// 
    /// To avoid an unnecessary proliferation of additional attributes with similar semantics but different names
    /// we recommend to re-use attributes from the 'recommended' list below first, and only introduce new attributes if nothing appropriate could be found.
    Module {
        /// True if the module is optimized.
        is_optimized | "isOptimized": Option<bool>,
        /// Unique identifier for the module.
        id | "id": Either<u64, String>,
        /// Version of Module.
        version | "version": Option<String>,
        /// Address range covered by this module.
        address_range | "addressRange": Option<String>,
        /// True if the module is considered 'user code' by a debugger that supports 'Just My Code'.
        is_user_code | "isUserCode": Option<bool>,
        /// A name of the module.
        name | "name": String,
        /// Logical full path to the symbol file. The exact definition is implementation defined.
        symbol_file_path | "symbolFilePath": Option<String>,
        /// optional but recommended attributes.
        /// always try to use these first before introducing additional attributes.
        /// 
        /// Logical full path to the module. The exact definition is implementation defined, but usually this would be a full path to the on-disk file for the module.
        path | "path": Option<String>,
        /// User understandable description of if symbols were found for the module (ex: 'Symbols Loaded', 'Symbols not found', etc.
        symbol_status | "symbolStatus": Option<String>,
        /// Module created or modified.
        date_time_stamp | "dateTimeStamp": Option<String>,
    }
);
