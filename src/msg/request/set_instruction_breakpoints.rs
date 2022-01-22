use crate::msg::dap_type::InstructionBreakpoint;

request2!(
    SetInstructionBreakpointsRequest {
        /// The instruction references of the breakpoints
        breakpoints | "breakpoints": Vec<InstructionBreakpoint>,
    }
);
