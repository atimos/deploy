use std::collections::HashMap;

pub type InstanceId = uuid::Uuid;

pub enum Node {
    Program {
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

pub struct Command {
    pub name: String,
    pub arguments: Option<Arguments>,
}

pub enum Arguments {
    Map(HashMap<String, String>),
    List(Vec<String>),
    String(String),
}

pub enum Location {
    Wasm { uri: String },
    Oci { repository: String, image: String },
}

pub enum ExecutionMode {
    Sequence,
    Parallel,
}

pub enum Status {
    Error,
    Success,
    Abort,
}
