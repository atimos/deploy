mod convert;

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Deserialize)]
pub struct Pipeline {
    pub pipeline: Node,
    #[serde(default)]
    pub units: HashMap<String, Node>,
}

#[derive(Clone, Deserialize)]
#[serde(untagged)]
pub enum Node {
    Command {
        #[serde(flatten)]
        command: Command,
        #[serde(flatten)]
        location: Location,
        #[serde(default)]
        description: String,
        #[serde(default)]
        run_on: Vec<Status>,
    },
    Commands {
        #[serde(rename = "cmds")]
        commands: Vec<Command>,
        #[serde(flatten)]
        location: Location,
        #[serde(default)]
        description: String,
        #[serde(default)]
        run_on: Vec<Status>,
    },
    DefaultList(Vec<Node>),
    List {
        list: Vec<Node>,
        #[serde(default)]
        description: String,
        #[serde(default)]
        mode: ExecutionMode,
        #[serde(default)]
        run_on: Vec<Status>,
    },
    One {
        #[serde(rename = "run")]
        node: Box<Node>,
        #[serde(default)]
        description: String,
        #[serde(default)]
        run_on: Vec<Status>,
    },
    Reference {
        id: String,
        arguments: Option<Arguments>,
        #[serde(default)]
        run_on: Vec<Status>,
    },
}

#[derive(Clone, Deserialize)]
pub enum ExecutionMode {
    #[serde(rename = "sequence")]
    Sequence,
    #[serde(rename = "parallel")]
    Parallel,
}

impl Default for ExecutionMode {
    fn default() -> Self {
        Self::Sequence
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
