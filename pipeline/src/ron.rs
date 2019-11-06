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
    Commands {
        cmd: Commands,
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
        args: Option<HashMap<String, String>>,
        #[serde(default)]
        run_on: Vec<Status>,
    },
}

#[allow(non_camel_case_types)]
#[derive(Clone, Deserialize)]
pub enum ExecutionMode {
    sequence,
    parallel,
}

impl Default for ExecutionMode {
    fn default() -> Self {
        Self::sequence
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Deserialize)]
pub enum Status {
    error,
    success,
    abort,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Deserialize)]
#[serde(tag = "type")]
pub enum Location {
    wasm { uri: String },
    oci { repo: String, image: String },
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
