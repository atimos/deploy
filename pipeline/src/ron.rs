mod convert;

use crate::data::Template;
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
        description: Template,
        #[serde(default)]
        run_on: Vec<Status>,
    },
    DefaultList(Vec<Node>),
    Nodes {
        list: Vec<Node>,
        #[serde(default)]
        description: Template,
        #[serde(default)]
        mode: ExecutionMode,
        #[serde(default)]
        run_on: Vec<Status>,
    },
    Reference {
        id: String,
        args: Option<HashMap<String, Template>>,
        #[serde(default)]
        run_on: Vec<Status>,
    },
}

#[allow(non_camel_case_types)]
#[derive(Clone, Deserialize)]
pub enum ExecutionMode {
    #[serde(rename = "sequence-stop-on-error")]
    SequenceStopOnError,
    sequence,
    parallel,
}

impl Default for ExecutionMode {
    fn default() -> Self {
        Self::SequenceStopOnError
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
    wasm { uri: Template },
    oci { repo: Template, image: Template },
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
    Map(HashMap<String, Template>),
    List(Vec<Template>),
    String(Template),
}
