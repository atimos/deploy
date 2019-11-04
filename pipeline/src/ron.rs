mod convert;

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Deserialize)]
pub struct Pipeline {
    #[serde(rename = "pipeline")]
    pub start: Node,
    #[serde(default)]
    pub units: HashMap<String, Node>,
}

#[derive(Clone, Deserialize)]
#[serde(untagged)]
pub enum Node {
    Commands {
        #[serde(rename = "cmd")]
        commands: Commands,
        #[serde(flatten)]
        location: Location,
        #[serde(default)]
        description: String,
        #[serde(default)]
        run_on: Vec<Status>,
    },
    DefaultList(Vec<Node>),
    Nodes {
        list: Vec<Node>,
        #[serde(default)]
        description: String,
        #[serde(default)]
        mode: ExecutionMode,
        #[serde(default)]
        run_on: Vec<Status>,
    },
    Reference {
        id: String,
        #[serde(rename = "args")]
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
#[serde(untagged)]
pub enum Commands {
    One(Command),
    Multiple(Vec<Command>),
}

#[derive(Clone, Deserialize)]
pub struct Command {
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
