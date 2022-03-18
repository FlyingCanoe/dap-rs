use crate::msg::dap_type::instruction_breakpoint::InstructionBreakpoint;
use crate::msg::dap_type::breakpoint::Breakpoint;

request!(
    /// Replaces all existing instruction breakpoints. Typically, instruction breakpoints would be set from a diassembly window. 
    /// To clear all instruction breakpoints, specify an empty array.
    /// When an instruction breakpoint is hit, a 'stopped' event (with reason 'instruction breakpoint') is generated.
    /// Clients should only call this request if the capability 'supportsInstructionBreakpoints' is true.
    SetInstructionBreakpointsRequest {
        /// The instruction references of the breakpoints
        breakpoints | "breakpoints": Vec<InstructionBreakpoint>,
    }
);

response!(
    /// Response to 'setInstructionBreakpoints' request
    SetInstructionBreakpointsResponse {
        /// Information about the breakpoints. The array elements correspond to the elements of the 'breakpoints' array.
        breakpoints | "breakpoints": Vec<Breakpoint>,
    }
);
