use std::collections::HashMap;

use serde::Deserialize;

pub type TaskId = String;
pub type Tasks = HashMap<TaskId, Task>;

#[derive(Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Command {
    #[serde(rename = "plugin")]
    Plugin {
        id: String,
        cmd: String,
        args: Option<Args>,
    },
    #[serde(rename = "task")]
    Task { id: TaskId, args: Option<Args> },
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum Args {
    Map(HashMap<String, String>),
    List(Vec<String>),
}

#[derive(Deserialize, Clone)]
pub struct Task {
    #[serde(default)]
    pub execution_mode: ExecutionMode,
    pub run: Vec<Command>,
    #[serde(rename = "run-before")]
    pub run_before: Option<Command>,
    #[serde(rename = "run-after")]
    pub run_after: Option<Command>,
    #[serde(rename = "run-after-success")]
    pub run_after_success: Option<Command>,
    #[serde(rename = "run-after-error")]
    pub run_after_error: Option<Command>,
}

#[derive(Deserialize)]
pub struct Step {
    pub description: String,
    #[serde(flatten)]
    pub task: Task,
}

#[derive(Deserialize, Copy, Clone)]
pub enum ExecutionMode {
    #[serde(rename = "sequence-stop-on-error")]
    SequenceStopOnError,
    #[serde(rename = "sequence-run-all")]
    SequenceRunAll,
    #[serde(rename = "parallel")]
    Parallel,
}

impl Default for ExecutionMode {
    fn default() -> Self {
        Self::SequenceStopOnError
    }
}

#[derive(Deserialize)]
pub struct Runner {
    pub name: String,
    pub args: Option<Args>,
}

#[derive(Deserialize)]
pub(crate) struct Pipeline {
    pub runners: Option<Vec<Runner>>,
    pub steps: Vec<Step>,
    #[serde(default)]
    pub tasks: Tasks,
}
