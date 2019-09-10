use derivative::Derivative;
use std::collections::HashMap;

pub type InstanceId = uuid::Uuid;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum Block {
    Program {
        id: InstanceId,
        description: Option<String>,
        commands: Vec<Command>,
        location: Location,
        arguments: Option<Arguments>,
    },
    List {
        description: Option<String>,
        list: Vec<Block>,
        mode: ExecutionMode,
        run_on: Vec<Status>,
        arguments: Option<Arguments>,
    },
    On {
        description: Option<String>,
        condition: Box<Block>,
        success: Option<Box<Block>>,
        error: Option<Box<Block>>,
        abort: Option<Box<Block>>,
        arguments: Option<Arguments>,
    },
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Command {
    pub name: String,
    pub arguments: Option<Arguments>,
}

#[derive(Derivative)]
#[derivative(Debug)]
pub enum Arguments {
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
