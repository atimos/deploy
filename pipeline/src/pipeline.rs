use std::collections::HashMap;
use uuid::Uuid;

pub type InstanceId = Uuid;

#[derive(Debug)]
pub struct Pipeline {
    pub commands: Struct,
}

#[derive(Debug)]
pub struct ArgumentKey {
    pub name: String,
}

#[derive(Debug)]
pub enum Struct {
    Commands {
        instance_id: InstanceId,
        description: Option<String>,
        arguments: Option<Arguments>,
        commands: Vec<Command>,
        location: Location,
    },
    Group {
        instance_id: InstanceId,
        description: Option<String>,
        arguments: Option<Arguments>,
        group: Vec<Struct>,
        mode: ExecutionMode,
        run_on_status: Vec<Status>,
    },
    On {
        instance_id: InstanceId,
        description: Option<String>,
        arguments: Option<Arguments>,
        condition: Box<Struct>,
        on_success: Option<Box<Struct>>,
        on_error: Option<Box<Struct>>,
        on_abort: Option<Box<Struct>>,
    },
}

#[derive(Debug)]
pub struct Command {
    name: String,
    arguments: Option<Arguments>
}

#[derive(Debug)]
pub enum Location {
    Wasm {
        uri: String,
    },
    Oci {
        repository: String,
        image: String,
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
pub enum Arguments {
    Map(HashMap<String, String>),
    List(Vec<String>),
    String(String),
}
