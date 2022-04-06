use crate::msg::dap_type::source::Source;
use either::Either;

dap_type_enum!(
    /// An optional hint for how to present this frame in the UI.
    /// A value of 'label' can be used to indicate that the frame is an artificial frame that is used as a visual label or separator. A value of 'subtle' can be used to change the appearance of a frame in a 'subtle' way.
    PresentationHint {
        Normal | "normal",
        Label | "label",
        Subtle | "subtle",
    }
);

dap_type_struct!(
    /// A Stackframe contains the source location.
    StackFrame {
        /// The optional source of the frame.
        source | "source": Option<Source>,
        /// The module associated with this frame, if any.
        module_id | "moduleId": Option<Either<u64, String>>,
        /// An identifier for the stack frame. It must be unique across all threads.
        /// This id can be used to retrieve the scopes of the frame with the 'scopesRequest' or to restart the execution of a stackframe.
        id | "id": u64,
        /// Optional memory reference for the current instruction pointer in this frame.
        instruction_pointer_reference | "instructionPointerReference": Option<String>,
        /// The line within the file of the frame. If source is null or doesn't exist, line is 0 and must be ignored.
        line | "line": u64,
        /// An optional end column of the range covered by the stack frame.
        end_column | "endColumn": Option<u64>,
        /// The name of the stack frame, typically a method name.
        name | "name": String,
        /// The column within the line. If source is null or doesn't exist, column is 0 and must be ignored.
        column | "column": u64,
        /// An optional hint for how to present this frame in the UI.
        /// A value of 'label' can be used to indicate that the frame is an artificial frame that is used as a visual label or separator. A value of 'subtle' can be used to change the appearance of a frame in a 'subtle' way.
        presentation_hint | "presentationHint": Option<PresentationHint>,
        /// An optional end line of the range covered by the stack frame.
        end_line | "endLine": Option<u64>,
        /// Indicates whether this frame can be restarted with the 'restart' request. Clients should only use this if the debug adapter supports the 'restart' request (capability 'supportsRestartRequest' is true).
        can_restart | "canRestart": Option<bool>,
    }
);

builder!(
    type BuildedType = StackFrame;

    StackFrameBuilder {
        /// optional field
        source: Option Source,
        end_column: Option u64,
        end_line: Option u64,
        can_restart: Option bool,
        module_id: Option Either<u64, String>,
        instruction_pointer_reference: Option String,
        presentation_hint: Option PresentationHint,
        /// non optional field
        id: u64,
        name: String,
        line: u64,
        column: u64,
    }
);
