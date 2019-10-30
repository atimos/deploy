use std::collections::HashMap;

pub type InstanceId = uuid::Uuid;

#[derive(Debug)]
pub enum Node {
    Commands {
        id: InstanceId,
        description: Option<String>,
        commands: Vec<Command>,
        location: Location,
        arguments: Option<Arguments>,
        run_on: Vec<Status>,
    },
    List {
        description: Option<String>,
        list: Vec<Node>,
        mode: ExecutionMode,
        run_on: Vec<Status>,
        arguments: Option<Arguments>,
    },
}

#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub arguments: Option<Arguments>,
}

#[derive(Debug)]
pub enum Arguments {
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
    Sequence,
    Parallel,
}

#[derive(Debug)]
pub enum Status {
    Error,
    Success,
    Abort,
}
