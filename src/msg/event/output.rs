use crate::msg::dap_type::source::Source;
use serde_json as json;

dap_type_enum!(
    /// The output category. If not specified or if the category is not understand by the client, 'console' is assumed.
    Category {
        Other,
        /// Show the output in the client's default message UI, e.g. a 'debug console'. This category should only be used for informational output from the debugger (as opposed to the debuggee).
        Console | "console",
        /// A hint for the client to show the ouput in the client's UI for important and highly visible information, e.g. as a popup notification. This category should only be used for important messages from the debugger (as opposed to the debuggee). Since this category value is a hint, clients might ignore the hint and assume the 'console' category.
        Important | "important",
        /// Show the output as normal program output from the debuggee.
        Stdout | "stdout",
        /// Show the output as error program output from the debuggee.
        Stderr | "stderr",
        /// Send the output to telemetry instead of showing it to the user.
        Telemetry | "telemetry",
    }
);

dap_type_enum!(
    /// Support for keeping an output log organized by grouping related messages.
    Group {
        /// Start a new group in expanded mode. Subsequent output events are members of the group and should be shown indented.
        /// The 'output' attribute becomes the name of the group and is not indented.
        Start | "start",
        /// Start a new group in collapsed mode. Subsequent output events are members of the group and should be shown indented (as soon as the group is expanded).
        /// The 'output' attribute becomes the name of the group and is not indented.
        StartCollapsed | "startCollapsed",
        /// End the current group and decreases the indentation of subsequent output events.
        /// A non empty 'output' attribute is shown as the unindented end of the group.
        End | "end",
    }
);

event!(
    /// The event indicates that the target has produced some output.
    OutputEvent | "output" {
        /// An optional source location where the output was produced.
        source | "source": Option<Source>,
        /// Optional data to report. For the 'telemetry' category the data will be sent to telemetry, for the other categories the data is shown in JSON format.
        data | "data": Option<json::Value>,
        /// The output category. If not specified or if the category is not understand by the client, 'console' is assumed.
        category | "category": Option<Category>,
        /// An optional source location line where the output was produced.
        line | "line": Option<u64>,
        /// The output to report.
        output | "output": String,
        /// Support for keeping an output log organized by grouping related messages.
        group | "group": Option<Group>,
        /// An optional source location column where the output was produced.
        column | "column": Option<u64>,
        /// If an attribute 'variablesReference' exists and its value is > 0, the output contains objects which can be retrieved by passing 'variablesReference' to the 'variables' request. The value should be less than or equal to 2147483647 (2^31-1).
        variables_reference | "variablesReference": Option<u64>,
    }
);
