mod convert;
pub mod error;

use std::convert::TryInto;
use std::collections::HashMap;

use serde::Deserialize;

pub fn parse(content: &[u8]) -> Result<crate::pipeline::Pipeline, error::Error> {
    Ok(toml::from_slice::<Pipeline>(content)?.try_into()?)
}

#[derive(Debug, Deserialize)]
pub struct Pipeline {
    pub steps: Vec<Unit>,
    #[serde(default)]
    pub units: HashMap<String, Unit>,
}

#[derive(Debug, Deserialize)]
pub struct Unit {
    pub description: Option<String>,
    pub args: Option<Vec<ArgumentKey>>,
    pub commands: Commands,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Commands {
    Single(Command),
    ConfiguredList {
        commands: Vec<Commands>,
        #[serde(default)]
        mode: ExecutionMode,
        #[serde(default)]
        run_on_status: Vec<Status>,
    },
    List(Vec<Commands>),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Command {
    #[serde(rename = "unit")]
    Unit { id: String, args: Option<Arguments> },
    #[serde(rename = "if")]
    If {
        condition: Box<Commands>,
        then: Box<Commands>,
        otherwise: Option<Box<Commands>>,
    },
    #[serde(rename = "wasm")]
    Wasm {
        uri: String,
        #[serde(flatten)]
        command: Option<ExternalCommand>,
        commands: Option<Vec<ExternalCommand>>,
    },
    #[serde(rename = "oci")]
    Oci {
        repository: String,
        image: String,
        #[serde(flatten)]
        command: Option<ExternalCommand>,
        commands: Option<Vec<ExternalCommand>>,
    },
}

#[derive(Debug, Deserialize)]
pub struct ExternalCommand {
    command: String,
    args: Option<Arguments>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Arguments {
    Map(HashMap<String, String>),
    List(Vec<String>),
    String(String),
}

#[derive(Debug, Deserialize)]
pub struct ArgumentKey {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub enum Status {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "abort")]
    Abort,
}

#[derive(Debug, Deserialize)]
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
