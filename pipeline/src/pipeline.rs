use std::collections::HashMap;

pub type InstanceId = uuid::Uuid;

pub enum Section {
    Program {
        id: InstanceId,
        description: Option<String>,
        commands: Vec<Command>,
        location: Location,
        arguments: Option<Arguments>,
    },
    List {
        description: Option<String>,
        list: Vec<Section>,
        mode: ExecutionMode,
        run_on: Vec<Status>,
        arguments: Option<Arguments>,
    },
    On {
        description: Option<String>,
        condition: Box<Section>,
        success: Option<Box<Section>>,
        error: Option<Box<Section>>,
        abort: Option<Box<Section>>,
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
    SequenceStopOnError,
    SequenceRunAll,
    Parallel,
}

pub enum Status {
    Error,
    Success,
    Abort,
}
