use super::*;
use crate::{data::Template, error::Error, pipeline as p};
use ron::de::Error as RonError;
use std::{collections::HashMap, convert::TryInto, result::Result as StdResult};

type Units = HashMap<String, Node>;
type CircularCheck<'a> = Vec<&'a str>;
type Result = StdResult<p::Node, Error>;

impl From<RonError> for Error {
    fn from(err: RonError) -> Self {
        Self::Syntax(err)
    }
}

impl TryInto<p::Node> for Pipeline {
    type Error = Error;

    fn try_into(self) -> Result {
        convert_node(&self.pipeline, &None, &self.units, Vec::new(), &[Status::success])
    }
}

fn convert_node(
    node: &Node,
    local_env: &Option<HashMap<String, Template>>,
    units: &Units,
    circular: CircularCheck<'_>,
    parent_run_on: &[Status],
) -> Result {
    Ok(match node {
        Node::Nodes { list, description, mode, run_on } => {
            let run_on = if run_on.is_empty() { parent_run_on } else { &run_on };
            p::Node::Nodes {
                description: convert_description(description),
                nodes: list
                    .iter()
                    .map(|item| convert_node(item, local_env, units, circular.clone(), run_on))
                    .collect::<StdResult<Vec<p::Node>, Error>>()?,
                mode: mode.into(),
                run_on: convert_run_on(run_on),
                local: local_env.clone().map(Into::into),
            }
        }
        Node::DefaultList(list) => p::Node::Nodes {
            description: convert_description(&Template::default()),
            nodes: list
                .iter()
                .map(|item| convert_node(item, local_env, units, circular.clone(), parent_run_on))
                .collect::<StdResult<Vec<p::Node>, Error>>()?,
            mode: p::ExecutionMode::Sequence.into(),
            run_on: convert_run_on(parent_run_on),
            local: local_env.clone().map(Into::into),
        },
        Node::Commands { cmd, location, description, run_on } => p::Node::Commands {
            id: p::InstanceId::new_v4(),
            description: convert_description(description),
            commands: convert_commands(cmd),
            location: location.into(),
            local: local_env.clone().map(Into::into),
            run_on: convert_run_on(if run_on.is_empty() { parent_run_on } else { &run_on }),
        },
        Node::Reference { id, local, run_on } => convert_reference(
            id,
            local,
            units,
            circular,
            if run_on.is_empty() { parent_run_on } else { &run_on },
        )?,
    })
}

fn convert_reference<'a>(
    id: &'a str,
    local_env: &Option<HashMap<String, Template>>,
    units: &Units,
    mut circular: CircularCheck<'a>,
    run_on: &[Status],
) -> Result {
    if circular.contains(&id) {
        circular.push(id);
        Err(Error::Recursion(circular.into_iter().map(String::from).collect()))
    } else if let Some(unit) = units.get(id) {
        circular.push(id);
        convert_node(unit, local_env, units, circular, run_on)
    } else {
        Err(Error::NotFound(id.to_owned()))
    }
}

fn convert_description(description: &Template) -> Option<Template> {
    if description.inner().is_empty() {
        None
    } else {
        Some(description.to_owned())
    }
}

fn convert_run_on(list: &[Status]) -> Vec<p::Status> {
    list.iter().map(Into::into).collect()
}

fn convert_commands(commands: &Commands) -> Vec<p::Command> {
    match commands {
        Commands::One(command) => vec![command.into()],
        Commands::Multiple(commands) => commands.iter().map(Into::into).collect(),
    }
}

impl Into<p::ExecutionMode> for &ExecutionMode {
    fn into(self) -> p::ExecutionMode {
        match self {
            ExecutionMode::SequenceStopOnError => p::ExecutionMode::SequenceStopOnError,
            ExecutionMode::sequence => p::ExecutionMode::Sequence,
            ExecutionMode::parallel => p::ExecutionMode::Parallel,
        }
    }
}

impl Into<p::Status> for &Status {
    fn into(self) -> p::Status {
        match self {
            Status::success => p::Status::Success,
            Status::error => p::Status::Error,
            Status::abort => p::Status::Abort,
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
            Location::wasm { uri } => p::Location::Wasm { uri: uri.to_owned() },
            Location::oci { repo, image } => {
                p::Location::Oci { repository: repo.to_owned(), image: image.to_owned() }
            }
        }
    }
}
