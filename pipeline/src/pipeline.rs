use std::collections::HashMap;

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
    Unit {
        id: String,
        args: Option<Arguments>,
    },
    Wasm {
        uri: String,
        command: String,
        args: Option<Arguments>,
    },
    Oci {
        repository: String,
        image: String,
        command: String,
        raw_command: bool,
        force_rebuild: bool,
        args: Option<Arguments>,
    },
}

#[derive(Debug)]
pub enum Arguments {
    Map(HashMap<String, String>),
    List(Vec<String>),
    String(String),
}
