use super::Url;
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Pipeline {
    pub steps: Vec<Step>,
    pub inline: HashMap<String, Inline>,
}

#[derive(Debug)]
pub struct Step {
    pub description: String,
    pub run: Commands,
    pub run_before: Option<Command>,
    pub run_after: Option<Command>,
    pub run_after_success: Option<Command>,
    pub run_after_error: Option<Command>,
}

#[derive(Debug)]
pub struct Command {
    pub id: CommandId,
    pub uri: Url,
    pub args: HashMap<String, Argument>,
}

#[derive(Debug)]
pub enum CommandId {
    Uuid(Uuid),
    Named(String),
}

#[derive(Debug)]
pub enum Commands {
    SequenceStopOnError(Vec<Command>),
    SequenceRunAll(Vec<Command>),
    Parallel(Vec<Command>),
}

#[derive(Debug)]
pub struct Inline {
    pub run: Commands,
    pub run_before: Option<Command>,
    pub run_after: Option<Command>,
    pub run_after_success: Option<Command>,
    pub run_after_error: Option<Command>,
}

#[derive(Debug)]
pub enum Argument {
    Map(HashMap<String, String>),
    List(Vec<String>),
    String(String),
}
