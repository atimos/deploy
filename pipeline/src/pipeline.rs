use std::collections::HashMap;
use uuid::Uuid;

pub type InstanceId = Uuid;
pub type Arguments = Option<HashMap<String, String>>;

#[derive(Debug)]
pub enum Pipeline {
    Program {
        instance_id: InstanceId,
        description: Option<String>,
        cmds: Vec<Command>,
        location: Location,
        args: Arguments,
    },
    List {
        instance_id: InstanceId,
        description: Option<String>,
        list: Vec<Pipeline>,
        mode: ExecutionMode,
        run_on: Vec<Status>,
        args: Arguments,
    },
    On {
        instance_id: InstanceId,
        description: Option<String>,
        condition: Box<Pipeline>,
        on_success: Option<Box<Pipeline>>,
        on_error: Option<Box<Pipeline>>,
        on_abort: Option<Box<Pipeline>>,
        args: Arguments,
    },
}

#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub args: Option<CommandArguments>,
}

#[derive(Debug)]
pub enum CommandArguments {
    Map(HashMap<String, String>),
    List(Vec<String>),
    String(String),
}

#[derive(Debug)]
pub enum Location {
    Wasm { uri: String },
    Oci { repository: String, image: String },
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
