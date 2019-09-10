mod convert;

use crate::{error, pipeline};
use derivative::Derivative;
use serde::Deserialize;
use std::{collections::HashMap, convert::TryInto};

pub use convert::Error;

pub fn parse(content: &[u8]) -> Result<pipeline::Block, error::Error> {
    Ok(ron::de::from_bytes::<Data>(content).map_err(Error::Parse)?.try_into()?)
}

#[derive(Derivative, Deserialize)]
#[derivative(Debug, Clone)]
pub struct Data {
    pub pipeline: Block,
    #[serde(default)]
    pub units: HashMap<String, Block>,
}

#[derive(Derivative, Deserialize)]
#[derivative(Debug, Clone)]
#[serde(untagged)]
pub enum Block {
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
    DefaultList(Vec<Block>),
    List {
        list: Vec<Block>,
        #[serde(default)]
        description: String,
        #[serde(default)]
        mode: ExecutionMode,
        #[serde(default)]
        run_on: Vec<Status>,
    },
    One {
        run: Box<Block>,
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
        condition: Box<Block>,
        #[serde(default)]
        description: String,
        #[serde(default)]
        on_success: Option<Box<Block>>,
        #[serde(default)]
        on_error: Option<Box<Block>>,
        #[serde(default)]
        on_abort: Option<Box<Block>>,
    },
}

#[derive(Derivative, Deserialize)]
#[derivative(Debug, Clone, Default)]
pub enum ExecutionMode {
    #[serde(rename = "sequence-stop-on-error")]
    #[derivative(Default)]
    SequenceStopOnError,
    #[serde(rename = "sequence-run-all")]
    SequenceRunAll,
    #[serde(rename = "parallel")]
    Parallel,
}

#[derive(Derivative, Deserialize)]
#[derivative(Debug, Clone)]
pub enum Status {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "abort")]
    Abort,
}

#[derive(Derivative, Deserialize)]
#[derivative(Debug, Clone)]
#[serde(tag = "type")]
pub enum Location {
    #[serde(rename = "wasm")]
    Wasm { uri: String },
    #[serde(rename = "oci")]
    Oci { repo: String, image: String },
}

#[derive(Derivative, Deserialize)]
#[derivative(Debug, Clone)]
pub struct Command {
    #[serde(rename = "cmd")]
    pub name: String,
    #[serde(default)]
    pub args: Option<Arguments>,
}

#[derive(Derivative, Deserialize)]
#[derivative(Debug, Clone)]
#[serde(untagged)]
pub enum Arguments {
    Map(HashMap<String, String>),
    List(Vec<String>),
    String(String),
}
