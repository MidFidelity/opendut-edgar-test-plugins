use opendut_edgar_plugin_api::plugin::{export, host, trace, debug, info, warn, error};
use opendut_edgar_plugin_api::plugin::host::{call_command, log, LogLevel};
use opendut_edgar_plugin_api::plugin::task::{Guest, Success, TaskFulfilled};

struct TestPlugin4;

impl Guest for TestPlugin4 {
    fn description() -> String {
        String::from("Test Plugin 4 - Tasfulfilled Err")
    }

    fn check_fulfilled() -> Result<TaskFulfilled, ()> {
        Err(())
    }

    fn execute() -> Result<Success, ()> {
        Ok(Success::message("This should never be reached!"))
    }
}

export!(TestPlugin4 with_types_in opendut_edgar_plugin_api::plugin::bindings);
