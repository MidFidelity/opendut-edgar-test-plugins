use opendut_edgar_plugin_api::plugin::{export, host, trace, debug, info, warn, error};
use opendut_edgar_plugin_api::plugin::host::{call_command, log, LogLevel};
use opendut_edgar_plugin_api::plugin::task::{Guest, Success, TaskFulfilled};

struct TestPlugin5;

impl Guest for TestPlugin5 {
    fn description() -> String {
        String::from("Test Plugin 5 - Execute Err")
    }

    fn check_fulfilled() -> Result<TaskFulfilled, ()> {
        Ok(TaskFulfilled::Unchecked)
    }

    fn execute() -> Result<Success, ()> {
        Err(())
    }
}

export!(TestPlugin5 with_types_in opendut_edgar_plugin_api::plugin::bindings);
