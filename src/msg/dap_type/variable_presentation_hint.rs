dap_type_enum!(
    /// Visibility of variable. Before introducing additional values, try to use the listed values.
    Visibility {
        Other,
        Public | "public",
        Private | "private",
        Protected | "protected",
        Internal | "internal",
        Final | "final",
    }
);

dap_type_enum!(
    /// The kind of variable. Before introducing additional values, try to use the listed values.
    Kind {
        Other,
        /// Indicates that the object is a property.
        Property | "property",
        /// Indicates that the object is a method.
        Method | "method",
        /// Indicates that the object is a class.
        Class | "class",
        /// Indicates that the object is data.
        Data | "data",
        /// Indicates that the object is an event.
        Event | "event",
        /// Indicates that the object is a base class.
        BaseClass | "baseClass",
        /// Indicates that the object is an inner class.
        InnerClass | "innerClass",
        /// Indicates that the object is an interface.
        Interface | "interface",
        /// Indicates that the object is the most derived class.
        MostDerivedClass | "mostDerivedClass",
        /// Indicates that the object is virtual, that means it is a synthetic object introducedby the
        /// adapter for rendering purposes, e.g. an index range for large arrays.
        Virtual | "virtual",
        /// Deprecated: Indicates that a data breakpoint is registered for the object. The 'hasDataBreakpoint' attribute should generally be used instead.
        DataBreakpoint | "dataBreakpoint",
    }
);

dap_type_struct!(
    /// Optional properties of a variable that can be used to determine how to render the variable in the UI.
    VariablePresentationHint {
        /// Set of attributes represented as an array of strings. Before introducing additional values, try to use the listed values.
        attributes | "attributes": Option<Vec<String>>,
        /// The kind of variable. Before introducing additional values, try to use the listed values.
        kind | "kind": Option<Kind>,
        /// Visibility of variable. Before introducing additional values, try to use the listed values.
        visibility | "visibility": Option<Visibility>,
    }
);
