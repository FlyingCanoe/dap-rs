use super::{ExceptionBreakMode, ExceptionPathSegment};

dap_type_struct!(
    ExceptionOptions {
        /// A path that selects a single or multiple exceptions in a tree. If 'path' is
        /// missing, the whole tree is selected.
        /// By convention the first segment of the path is a category that is used to
        /// group exceptions in the UI.
        path | "path": Option<Vec<ExceptionPathSegment>>,

        /// Condition when a thrown exception should result in a break.
        break_mode | "breakMode": ExceptionBreakMode,
    }
);
