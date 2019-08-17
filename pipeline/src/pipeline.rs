use std::collections::HashMap;
use uuid::Uuid;

pub type InstanceId = Uuid;

#[derive(Debug)]
pub struct Pipeline {
    pub steps: Vec<Unit>,
    pub units: Units,
}

pub type Units = HashMap<String, Unit>;

#[derive(Debug)]
pub struct Unit {
    pub description: Option<String>,
    pub commands: Commands,
    pub args: Option<Vec<ArgumentKey>>,
}

#[derive(Debug)]
pub struct ArgumentKey {
    pub name: String,
}

#[derive(Debug)]
pub enum Commands {
    Single(Command),
    Multiple {
        commands: Vec<Commands>,
        mode: ExecutionMode,
        run_on_status: Vec<Status>,
    },
}

#[derive(Debug)]
pub enum ExecutionMode {
    SequenceStopOnError,
    SequenceRunAll,
    Parallel,
}

#[derive(Debug)]
pub enum Status {
    Error,
    Success,
    Abort,
}

#[derive(Debug)]
pub enum Command {
    If {
        instance_id: InstanceId,
        condition: Box<Commands>,
        then: Box<Commands>,
        otherwise: Option<Box<Commands>>,
    },
    Unit {
        instance_id: InstanceId,
        id: String,
        args: Option<HashMap<String, String>>,
    },
    Wasm {
        instance_id: InstanceId,
        uri: String,
        commands: Vec<ExternalCommand>,
    },
    Oci {
        instance_id: InstanceId,
        repository: String,
        image: String,
        commands: Vec<ExternalCommand>,
    },
}

#[derive(Debug)]
pub struct ExternalCommand {
    pub command: String,
    pub args: Option<Arguments>,
}

#[derive(Debug)]
pub enum Arguments {
    Map(HashMap<String, String>),
    List(Vec<String>),
    String(String),
}
