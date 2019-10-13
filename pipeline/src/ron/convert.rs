use super::*;
use crate::{error::Error, pipeline as p};
use ron::de::Error as RonError;
use std::{collections::HashMap, convert::TryInto, result::Result as StdResult};

type Units = HashMap<String, Node>;
type CircularCheck<'a> = Vec<&'a str>;
type Args = Option<Arguments>;
type Result = StdResult<p::Node, Error>;

impl From<RonError> for Error {
    fn from(err: RonError) -> Self {
        Self::Syntax(err)
    }
}

impl TryInto<p::Node> for Pipeline {
    type Error = Error;

    fn try_into(self) -> Result {
        convert_node(&self.pipeline, &None, &self.units, Vec::new(), &vec![Status::Success])
    }
}

fn convert_node(
    node: &Node,
    args: &Args,
    units: &Units,
    circular: CircularCheck<'_>,
    parent_run_on: &[Status],
) -> Result {
    Ok(match node {
        Node::One { node, description, run_on } => {
            let run_on = if run_on.is_empty() { parent_run_on } else { &run_on };
            p::Node::List {
                description: convert_description(description),
                list: vec![convert_node(&*node, args, units, circular, run_on)?],
                mode: p::ExecutionMode::Sequence,
                run_on: convert_run_on(run_on),
                arguments: args.as_ref().map(Into::into),
            }
        }
        Node::List { list, description, mode, run_on } => {
            let run_on = if run_on.is_empty() { parent_run_on } else { &run_on };
            p::Node::List {
                description: convert_description(description),
                list: list
                    .iter()
                    .map(|item| convert_node(item, args, units, circular.clone(), run_on))
                    .collect::<StdResult<Vec<p::Node>, Error>>()?,
                mode: mode.into(),
                run_on: convert_run_on(run_on),
                arguments: args.as_ref().map(Into::into),
            }
        }
        Node::DefaultList(list) => p::Node::List {
            description: None,
            list: list
                .iter()
                .map(|item| convert_node(item, args, units, circular.clone(), parent_run_on))
                .collect::<StdResult<Vec<p::Node>, Error>>()?,
            mode: p::ExecutionMode::Sequence,
            run_on: convert_run_on(parent_run_on),
            arguments: args.as_ref().map(Into::into),
        },
        Node::Command { command, location, description, run_on } => p::Node::Program {
            id: p::InstanceId::new_v4(),
            description: convert_description(description),
            commands: vec![command.into()],
            location: location.into(),
            arguments: args.as_ref().map(Into::into),
            run_on: convert_run_on(if run_on.is_empty() { parent_run_on } else { &run_on }),
        },
        Node::Commands { commands, location, description, run_on } => p::Node::Program {
            id: p::InstanceId::new_v4(),
            description: convert_description(description),
            commands: commands.iter().map(Into::into).collect(),
            location: location.into(),
            arguments: args.as_ref().map(Into::into),
            run_on: convert_run_on(if run_on.is_empty() { parent_run_on } else { &run_on }),
        },
        Node::Reference { id, arguments, run_on } => convert_reference(
            id,
            arguments,
            units,
            circular,
            if run_on.is_empty() { parent_run_on } else { &run_on },
        )?,
    })
}

fn convert_reference<'a>(
    id: &'a str,
    args: &Args,
    units: &Units,
    mut circular: CircularCheck<'a>,
    run_on: &[Status],
) -> Result {
    if circular.contains(&id.as_ref()) {
        circular.push(id);
        Err(Error::Recursion(circular.into_iter().map(String::from).collect()))
    } else if let Some(unit) = units.get(id) {
        circular.push(id);
        convert_node(unit, args, units, circular, run_on)
    } else {
        return Err(Error::NotFound(id.to_owned()));
    }
}

fn convert_description(description: &str) -> Option<String> {
    if description.is_empty() {
        None
    } else {
        Some(description.to_owned())
    }
}

fn convert_run_on(list: &[Status]) -> Vec<p::Status> {
    list.iter().map(Into::into).collect()
}

impl Into<p::ExecutionMode> for &ExecutionMode {
    fn into(self) -> p::ExecutionMode {
        match self {
            ExecutionMode::Sequence => p::ExecutionMode::Sequence,
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
        p::Command { name: self.name.to_owned(), arguments: self.args.as_ref().map(Into::into) }
    }
}

impl Into<p::Arguments> for &Arguments {
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
