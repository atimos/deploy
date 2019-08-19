mod convert;

use crate::{error, pipeline};
use serde::Deserialize;
use std::collections::HashMap;
use std::convert::TryInto;

pub use convert::Error;

pub fn parse(content: &[u8]) -> Result<pipeline::Pipeline, error::Error> {
    Ok(ron::de::from_bytes::<Data>(content)
        .map_err(Error::Parse)?
        .try_into()?)
}

pub type Arguments = Option<HashMap<String, String>>;

#[derive(Debug, Clone, Deserialize)]
pub struct Data {
    pub pipeline: Pipeline,
    #[serde(default)]
    pub units: HashMap<String, Pipeline>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Pipeline {
    ProgramSingleCommand {
        #[serde(flatten)]
        cmd: Command,
        #[serde(flatten)]
        location: Location,
        #[serde(default)]
        description: String,
    },
    ProgramMultipleCommands {
        cmds: Vec<Command>,
        #[serde(flatten)]
        location: Location,
        #[serde(default)]
        description: String,
    },
    DefaultList(Vec<Pipeline>),
    List {
        list: Vec<Pipeline>,
        #[serde(default)]
        description: String,
        #[serde(default)]
        mode: ExecutionMode,
        #[serde(default)]
        run_on: Vec<Status>,
    },
    One {
        run: Box<Pipeline>,
        #[serde(default)]
        description: String,
        #[serde(default)]
        run_on: Vec<Status>,
    },
    Reference {
        id: String,
        args: Arguments,
    },
    On {
        condition: Box<Pipeline>,
        #[serde(default)]
        description: String,
        #[serde(default)]
        on_success: Option<Box<Pipeline>>,
        #[serde(default)]
        on_error: Option<Box<Pipeline>>,
        #[serde(default)]
        on_abort: Option<Box<Pipeline>>,
    },
}

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
pub enum Status {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "abort")]
    Abort,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum Location {
    #[serde(rename = "wasm")]
    Wasm { uri: String },
    #[serde(rename = "oci")]
    Oci { repo: String, image: String },
}

#[derive(Debug, Clone, Deserialize)]
pub struct Command {
    pub cmd: String,
    #[serde(default)]
    pub args: Option<CommandArguments>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum CommandArguments {
    Map(HashMap<String, String>),
    List(Vec<String>),
    String(String),
}
