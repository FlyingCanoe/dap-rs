use crate::msg::dap_type::invalidated_areas::InvalidatedAreas;

event!(
    /// This event signals that some state in the debug adapter has changed and requires that the client needs to re-render the data snapshot previously requested.
    /// Debug adapters do not have to emit this event for runtime changes like stopped or thread events because in that case the client refetches the new state anyway. But the event can be used for example to refresh the UI after rendering formatting has changed in the debug adapter.
    /// This event should only be sent if the debug adapter has received a value true for the 'supportsInvalidatedEvent' capability of the 'initialize' request.
    InvalidatedEvent {
        /// If specified, the client only needs to refetch data related to this thread.
        thread_id | "threadId": Option<u64>,
        /// If specified, the client only needs to refetch data related to this stack frame (and the 'threadId' is ignored).
        stack_frame_id | "stackFrameId": Option<u64>,
        /// Optional set of logical areas that got invalidated. This property has a hint characteristic: a client can only be expected to make a 'best effort' in honouring the areas but there are no guarantees. If this property is missing, empty, or if values are not understand the client should assume a single value 'all'.
        areas | "areas": Option<Vec<InvalidatedAreas>>,
    }
);
