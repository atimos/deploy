use crate::data::Template;
use std::collections::HashMap;

pub type InstanceId = uuid::Uuid;

#[derive(Debug)]
pub enum Node {
    Commands {
        id: InstanceId,
        description: Option<Template>,
        commands: Vec<Command>,
        location: Location,
        run_on: Vec<Status>,
        local: Option<Environment>,
    },
    Nodes {
        description: Option<Template>,
        nodes: Vec<Node>,
        mode: ExecutionMode,
        run_on: Vec<Status>,
        local: Option<Environment>,
    },
}

pub type Environment = HashMap<String, Template>;

#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub arguments: Option<Arguments>,
}

#[derive(Debug)]
pub enum Arguments {
    Map(HashMap<String, Template>),
    List(Vec<Template>),
    String(Template),
}

#[derive(Debug)]
pub enum Location {
    Wasm { uri: Template },
    Oci { repository: Template, image: Template },
}

#[derive(Debug)]
pub enum ExecutionMode {
    SequenceStopOnError,
    Sequence,
    Parallel,
}

#[derive(Debug)]
pub enum Status {
    Error,
    Success,
    Abort,
}
