use crate::msg::dap_type::{EvaluateCtx, ValueFormat};
use crate::utils::{parse_optional_u64, parse_string};

request2!(
    EvaluateRequest {
        /// The expression to evaluate.
        expression | "expression": String = parse_string,

        /// Evaluate the expression in the scope of this stack frame. If not specified,
        /// the expression is evaluated in the global scope.
        frame_id | "frameId": Option<u64> = parse_optional_u64,

        /// The context in which the evaluate request is run.
        /// Values:
        /// 'watch': evaluate is run in a watch.
        /// 'repl': evaluate is run from REPL console.
        /// 'hover': evaluate is run from a data hover.
        /// 'clipboard': evaluate is run to generate the value that will be stored in
        /// the clipboard.
        /// The attribute is only honored by a debug adapter if the capability
        /// 'supportsClipboardContext' is true.
        /// etc.
        context | "context": Option<EvaluateCtx> = EvaluateCtx::parse_option,

        /// Specifies details on how to format the Evaluate result.
        /// The attribute is only honored by a debug adapter if the capability
        /// 'supportsValueFormattingOptions' is true.
        format | "format": Option<ValueFormat> = ValueFormat::parse_option,
    }
);
