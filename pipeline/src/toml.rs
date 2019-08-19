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
    pub steps: Vec<Step>,
    #[serde(default)]
    pub units: HashMap<String, Unit>,
}

#[derive(Debug, Deserialize)]
pub struct Step {
    pub description: Option<String>,
    pub args: Option<Vec<ArgumentKey>>,
    pub commands: ControlStructure,
}

#[derive(Debug, Deserialize)]
pub struct Unit {
    pub description: Option<String>,
    pub args: Option<Vec<ArgumentKey>>,
    pub commands: ControlStructure,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Command {
    Wasm {
        uri: String,
        #[serde(flatten)]
        command: Option<ExternalCommand>,
        commands: Option<Vec<ExternalCommand>>,
    },
    Oci {
        repository: String,
        image: String,
        #[serde(flatten)]
        command: Option<ExternalCommand>,
        commands: Option<Vec<ExternalCommand>>,
    },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ControlStructure {
    #[serde(rename = "command")]
    Command(Command),
    #[serde(rename = "inline")]
    Inline(Box<ControlStructure>),
    #[serde(rename = "list")]
    List {
        commands: Vec<ControlStructure>,
        mode: ExecutionMode,
        run_on_status: Vec<Status>,
    },
    #[serde(rename = "on")]
    On {
        condition: Box<ControlStructure>,
        success: Option<Box<ControlStructure>>,
        error: Option<Box<ControlStructure>>,
        abort: Option<Box<ControlStructure>>,
    },
    #[serde(rename = "ref")]
    Reference {
        id: String,
        args: Option<HashMap<String, String>>,
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
