use derivative::Derivative;
use std::collections::HashMap;
use uuid::Uuid;

pub type InstanceId = Uuid;
pub type Arguments = Option<HashMap<String, String>>;

#[derive(Derivative)]
#[derivative(Debug)]
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
        cond: Box<Pipeline>,
        success: Option<Box<Pipeline>>,
        error: Option<Box<Pipeline>>,
        abort: Option<Box<Pipeline>>,
        args: Arguments,
    },
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Command {
    pub name: String,
    pub args: Option<CommandArguments>,
}

#[derive(Derivative)]
#[derivative(Debug)]
pub enum CommandArguments {
    #[derivative(Debug = "transparent")]
    Map(HashMap<String, String>),
    #[derivative(Debug = "transparent")]
    List(Vec<String>),
    #[derivative(Debug = "transparent")]
    String(String),
}

#[derive(Derivative)]
#[derivative(Debug)]
pub enum Location {
    Wasm { uri: String },
    Oci { repository: String, image: String },
}

#[derive(Derivative)]
#[derivative(Debug)]
pub enum ExecutionMode {
    SequenceStopOnError,
    SequenceRunAll,
    Parallel,
}

#[derive(Derivative)]
#[derivative(Debug)]
pub enum Status {
    Error,
    Success,
    Abort,
}
