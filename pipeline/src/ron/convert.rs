use super::*;
use crate::{error::ParseError, pipeline as p};
use ron::de::Error as RonError;
use std::{collections::HashMap, convert::TryInto, result::Result as StdResult};

type Units = HashMap<String, Node>;
type Used<'a> = Vec<&'a str>;
type Args = Option<Arguments>;
type Result = StdResult<p::Node, ParseError>;

impl From<RonError> for ParseError {
    fn from(err: RonError) -> Self {
        Self::Syntax(err)
    }
}

impl TryInto<p::Node> for Pipeline {
    type Error = ParseError;

    fn try_into(self) -> Result {
        convert_block(&self.pipeline, None, &self.units, Vec::new())
    }
}

fn convert_block<'a>(block: &'a Node, args: Args, units: &'a Units, used: Used<'a>) -> Result {
    Ok(match block {
        Node::One { run, description, run_on } => p::Node::List {
            description: get_description(description),
            list: vec![convert_block(&*run, args.clone(), units, used.clone())?],
            mode: p::ExecutionMode::SequenceStopOnError,
            run_on: convert_status_list(run_on),
            arguments: args.map(Into::into),
        },
        Node::List { list, description, mode, run_on } => p::Node::List {
            description: get_description(description),
            list: list
                .iter()
                .map(|item| convert_block(item, args.clone(), units, used.clone()))
                .collect::<StdResult<Vec<p::Node>, ParseError>>()?,
            mode: mode.into(),
            run_on: convert_status_list(run_on),
            arguments: args.map(Into::into),
        },
        Node::DefaultList(list) => p::Node::List {
            description: None,
            list: list
                .iter()
                .map(|item| convert_block(item, args.clone(), units, used.clone()))
                .collect::<StdResult<Vec<p::Node>, ParseError>>()?,
            mode: p::ExecutionMode::SequenceStopOnError,
            run_on: convert_status_list(&Vec::new()),
            arguments: args.map(Into::into),
        },
        Node::On { condition, description, on_success, on_error, on_abort } => p::Node::On {
            description: get_description(description),
            condition: Box::new(convert_block(&*condition, args.clone(), units, used.clone())?),
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
            arguments: args.map(Into::into),
        },
        Node::Command { command, location, description } => p::Node::Program {
            id: p::InstanceId::new_v4(),
            description: get_description(description),
            commands: vec![command.into()],
            location: location.into(),
            arguments: args.map(Into::into),
        },
        Node::Commands { commands, location, description } => p::Node::Program {
            id: p::InstanceId::new_v4(),
            description: get_description(description),
            commands: commands.iter().map(Into::into).collect(),
            location: location.into(),
            arguments: args.map(Into::into),
        },
        Node::Reference { id, arguments } => convert_reference(id, arguments, units, used)?,
    })
}

fn convert_reference<'a>(id: &'a str, args: &Args, units: &Units, mut used: Used<'a>) -> Result {
    if used.contains(&id.as_ref()) {
        used.push(id);
        Err(ParseError::Recursion(used.into_iter().map(String::from).collect()))
    } else if let Some(unit) = units.get(id) {
        used.push(id);
        convert_block(unit, args.clone(), units, used)
    } else {
        return Err(ParseError::NotFound(id.to_owned()));
    }
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
        p::Command { name: self.name.to_owned(), arguments: self.args.clone().map(Into::into) }
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
