mod convert;

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Deserialize)]
pub struct Pipeline {
    pub pipeline: Section,
    #[serde(default)]
    pub units: HashMap<String, Section>,
}

#[derive(Clone, Deserialize)]
#[serde(untagged)]
pub enum Section {
    Command {
        #[serde(flatten)]
        command: Command,
        #[serde(flatten)]
        location: Location,
        #[serde(default)]
        description: String,
    },
    Commands {
        #[serde(rename = "cmds")]
        commands: Vec<Command>,
        #[serde(flatten)]
        location: Location,
        #[serde(default)]
        description: String,
    },
    DefaultList(Vec<Section>),
    List {
        list: Vec<Section>,
        #[serde(default)]
        description: String,
        #[serde(default)]
        mode: ExecutionMode,
        #[serde(default)]
        run_on: Vec<Status>,
    },
    One {
        run: Box<Section>,
        #[serde(default)]
        description: String,
        #[serde(default)]
        run_on: Vec<Status>,
    },
    Reference {
        id: String,
        arguments: Option<Arguments>,
    },
    On {
        condition: Box<Section>,
        #[serde(default)]
        description: String,
        #[serde(default)]
        on_success: Option<Box<Section>>,
        #[serde(default)]
        on_error: Option<Box<Section>>,
        #[serde(default)]
        on_abort: Option<Box<Section>>,
    },
}

#[derive(Clone, Deserialize)]
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

#[derive(Clone, Deserialize)]
pub enum Status {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "abort")]
    Abort,
}

#[derive(Clone, Deserialize)]
#[serde(tag = "type")]
pub enum Location {
    #[serde(rename = "wasm")]
    Wasm { uri: String },
    #[serde(rename = "oci")]
    Oci { repo: String, image: String },
}

#[derive(Clone, Deserialize)]
pub struct Command {
    #[serde(rename = "cmd")]
    pub name: String,
    #[serde(default)]
    pub args: Option<Arguments>,
}

#[derive(Clone, Deserialize)]
#[serde(untagged)]
pub enum Arguments {
    Map(HashMap<String, String>),
    List(Vec<String>),
    String(String),
}
