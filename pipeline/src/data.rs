use std::collections::HashMap;

use uuid::Uuid;
use serde::Deserialize;

use super::Url;

#[derive(Deserialize, Debug)]
pub struct Pipeline {
    pub steps: Vec<Step>,
    #[serde(default)]
    pub inline: HashMap<String, Inline>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub struct Command {
    #[serde(default)]
    pub id: CommandId,
    pub uri: Url,
    #[serde(default)]
    pub args: HashMap<String, Argument>,
}

#[derive(Deserialize, Clone, Debug)]
pub enum CommandId {
    Uuid(Uuid),
    Named(String),
}

impl Default for CommandId {
    fn default() -> Self {
        Self::Uuid(Uuid::new_v4())
    }
}

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Argument {
    Map(HashMap<String, String>),
    List(Vec<String>),
    String(String),
}

#[derive(Deserialize, Clone, Debug)]
pub struct ArgumentKey {
    pub name: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Inline {
    #[serde(default)]
    pub args: Vec<ArgumentKey>,
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

#[derive(Deserialize, Debug)]
pub struct Step {
    pub description: String,
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

#[derive(Deserialize, Copy, Clone, Debug)]
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
