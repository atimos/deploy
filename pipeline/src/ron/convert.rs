use super::*;
use crate::pipeline as p;
use derivative::Derivative;
use ron::de::Error as RonError;
use std::{collections::HashMap, convert::TryInto};

type Units = HashMap<String, Pipeline>;
type Used<'a> = Vec<&'a str>;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum Error {
    Parse(RonError),
    NotFound(String),
    Recursion(Vec<String>),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Parse(err) => Some(err),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(err) => write!(f, "{}", err),
            Self::NotFound(uri) => write!(f, "Could not find unit \"{}\"", uri),
            Self::Recursion(name_list) => {
                write!(f, "Recursion found in {}", name_list.join(" -> "))
            }
        }
    }
}

impl From<RonError> for Error {
    fn from(err: RonError) -> Self {
        Self::Parse(err)
    }
}

impl TryInto<p::Pipeline> for Data {
    type Error = Error;

    fn try_into(self) -> Result<p::Pipeline, Error> {
        convert_block(&self.pipeline, None, &self.units, Vec::new())
    }
}

fn convert_block<'a>(
    block: &'a Pipeline,
    args: Option<Arguments>,
    units: &'a Units,
    mut used: Used<'a>,
) -> Result<p::Pipeline, Error> {
    Ok(match block {
        Pipeline::One { run, description, run_on } => p::Pipeline::List {
            id: p::Id::new_v4(),
            description: get_description(description),
            list: vec![convert_block(&*run, args.clone(), units, used.clone())?],
            mode: p::ExecutionMode::SequenceStopOnError,
            run_on: convert_status_list(run_on),
            args: args.map(Into::into),
        },
        Pipeline::List { list, description, mode, run_on } => p::Pipeline::List {
            id: p::Id::new_v4(),
            description: get_description(description),
            list: list
                .iter()
                .map(|item| convert_block(item, args.clone(), units, used.clone()))
                .collect::<Result<Vec<p::Pipeline>, Error>>()?,
            mode: mode.into(),
            run_on: convert_status_list(run_on),
            args: args.map(Into::into),
        },
        Pipeline::DefaultList(list) => p::Pipeline::List {
            id: p::Id::new_v4(),
            description: None,
            list: list
                .iter()
                .map(|item| convert_block(item, args.clone(), units, used.clone()))
                .collect::<Result<Vec<p::Pipeline>, Error>>()?,
            mode: p::ExecutionMode::SequenceStopOnError,
            run_on: convert_status_list(&Vec::new()),
            args: args.map(Into::into),
        },
        Pipeline::On { condition, description, on_success, on_error, on_abort } => {
            p::Pipeline::On {
                id: p::Id::new_v4(),
                description: get_description(description),
                cond: Box::new(convert_block(&*condition, args.clone(), units, used.clone())?),
                success: on_success
                    .as_ref()
                    .map(|block| convert_block(&*block, args.clone(), units, used.clone()))
                    .transpose()?
                    .map(Box::new),
                error: on_error
                    .as_ref()
                    .map(|block| convert_block(&*block, args.clone(), units, used.clone()))
                    .transpose()?
                    .map(Box::new),
                abort: on_abort
                    .as_ref()
                    .map(|block| convert_block(&*block, args.clone(), units, used.clone()))
                    .transpose()?
                    .map(Box::new),
                args: args.map(Into::into),
            }
        }
        Pipeline::ProgramSingleCommand { cmd, location, description } => p::Pipeline::Program {
            id: p::Id::new_v4(),
            description: get_description(description),
            cmds: vec![cmd.into()],
            location: location.into(),
            args: args.map(Into::into),
        },
        Pipeline::ProgramMultipleCommands { cmds, location, description } => p::Pipeline::Program {
            id: p::Id::new_v4(),
            description: get_description(description),
            cmds: cmds.iter().map(Into::into).collect(),
            location: location.into(),
            args: args.map(Into::into),
        },
        Pipeline::Reference { id, args } => {
            if used.contains(&id.as_ref()) {
                used.push(id);
                return Err(Error::Recursion(used.into_iter().map(String::from).collect()));
            }

            if let Some(unit) = units.get(id) {
                used.push(id);
                convert_block(unit, args.clone(), units, used)?
            } else {
                return Err(Error::NotFound(id.to_owned()));
            }
        }
    })
}

fn get_description(description: &str) -> Option<String> {
    if description.is_empty() {
        None
    } else {
        Some(description.to_owned())
    }
}

fn convert_status_list(list: &[Status]) -> Vec<p::Status> {
    let mut list: Vec<p::Status> = list.iter().map(Into::into).collect();
    if list.is_empty() {
        list.push(p::Status::Success);
    }
    list
}

impl Into<p::ExecutionMode> for &ExecutionMode {
    fn into(self) -> p::ExecutionMode {
        match self {
            ExecutionMode::SequenceStopOnError => p::ExecutionMode::SequenceStopOnError,
            ExecutionMode::SequenceRunAll => p::ExecutionMode::SequenceRunAll,
            ExecutionMode::Parallel => p::ExecutionMode::Parallel,
        }
    }
}

impl Into<p::Status> for &Status {
    fn into(self) -> p::Status {
        match self {
            Status::Success => p::Status::Success,
            Status::Error => p::Status::Error,
            Status::Abort => p::Status::Abort,
        }
    }
}

impl Into<p::Command> for &Command {
    fn into(self) -> p::Command {
        p::Command { name: self.cmd.to_owned(), args: self.args.clone().map(Into::into) }
    }
}

impl Into<p::Arguments> for Arguments {
    fn into(self) -> p::Arguments {
        match self {
            Arguments::Map(map) => p::Arguments::Map(map.to_owned()),
            Arguments::List(list) => p::Arguments::List(list.to_owned()),
            Arguments::String(string) => p::Arguments::String(string.to_owned()),
        }
    }
}

impl Into<p::Location> for &Location {
    fn into(self) -> p::Location {
        match self {
            Location::Wasm { uri } => p::Location::Wasm { uri: uri.to_owned() },
            Location::Oci { repo, image } => {
                p::Location::Oci { repository: repo.to_owned(), image: image.to_owned() }
            }
        }
    }
}
