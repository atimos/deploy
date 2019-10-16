use pipeline::Arguments as PipelineArguments;
use serde::Serialize;
use std::{collections::HashMap, path::PathBuf};

pub type Variables = HashMap<String, String>;

#[derive(Clone)]
pub struct Environment {
    root: PathBuf,
    pub variables: Variables,
}

impl Environment {
    pub fn new(path: PathBuf) -> Self {
        Environment { root: path, variables: HashMap::new() }
    }
}

#[derive(Serialize)]
pub enum Arguments {
    Map(HashMap<String, String>),
    List(Vec<String>),
    String(String),
}

impl From<PipelineArguments> for Arguments {
    fn from(args: PipelineArguments) -> Self {
        match args {
            PipelineArguments::Map(map) => Self::Map(map),
            PipelineArguments::List(list) => Self::List(list),
            PipelineArguments::String(string) => Self::String(string),
        }
    }
}
