use crate::msg::dap_type::{EvaluateCtx, ValueFormat};

request!(
    EvaluateRequest {
        optional_args = false;
        u64 {},
        Option<u64> {
            /// Evaluate the expression in the scope of this stack frame. If not specified,
            /// the expression is evaluated in the global scope.
            frame_id: "frameId",
        },
        Option<bool> {},
        String {
            /// The expression to evaluate.
            expression: "expression",
        },
        Option<String> {},
        Option<json::Value> {},
        Custom {
            {
                type = EvaluateCtx;
                closure = EvaluateCtx::parse;
                context: "context";
            },
            {
                type = ValueFormat;
                closure = ValueFormat::parse;
                /// Specifies details on how to format the Evaluate result.
                /// The attribute is only honored by a debug adapter if the capability
                /// 'supportsValueFormattingOptions' is true.
                format: "format";
            },
        },
        Option<Custom> {},
    }
);
