﻿request!(
    /// The request suspends the debuggee.
    /// The debug adapter first sends the response and then a 'stopped' event (with reason 'pause') after the thread has been paused successfully.
    PauseRequest {
        /// Pause execution for this thread.
        thread_id | "threadId": u64,
    }
);

response!(
    /// Response to 'pause' request. This is just an acknowledgement, so no body field is required.
    PauseResponse {}
);
