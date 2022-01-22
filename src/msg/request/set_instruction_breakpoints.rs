use crate::msg::dap_type::InstructionBreakpoint;

request!(
    SetInstructionBreakpointsRequest {
        /// The instruction references of the breakpoints
        breakpoints | "breakpoints": Vec<InstructionBreakpoint> = InstructionBreakpoint::parse_vec,
    }
);
