use crate::{environment::Environment, error::Error};
use pipeline::{data::Template, Command, InstanceId, Location, Node};
use std::{collections::HashMap, result::Result as StdResult};

pub type Result = StdResult<Vec<Output>, Error>;

#[derive(Debug)]
pub struct Output {
    status: Status,
    err: Vec<u8>,
    ok: Vec<u8>,
}

#[derive(Debug)]
pub enum Status {
    Error,
    Success,
    Abort,
}

pub struct Programs {
    inner: HashMap<InstanceId, Entry>,
}

impl Programs {
    pub fn load(pipeline: &Node) -> StdResult<Self, Error> {
        let mut programs = HashMap::new();
        get_references(pipeline, &mut programs);
        Ok(Programs { inner: programs })
    }

    pub fn run(&self, id: &InstanceId, cmds: &[Command], env: &mut Environment) -> Result {
        match self.inner.get(id) {
            Some(entry) => match entry {
                Entry::Reference(reference) => Ok(reference.load(id, env)?.run(cmds, env)),
                Entry::Program(program) => Ok(program.run(cmds, env)),
            },
            None => unreachable!(),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

enum Entry {
    Reference(Reference),
    Program(Program),
}

#[derive(Debug)]
pub enum Reference {
    Wasm { uri: Template },
    Oci { repository: Template, image: Template },
}

impl Reference {
    fn load(&self, id: &InstanceId, env: &Environment) -> StdResult<Program, Error> {
        match self {
            Self::Wasm { uri } => Ok(Program::Wasm { binary: Vec::new() }),
            Self::Oci { repository, image } => Ok(Program::Oci { container: String::new() }),
        }
    }
}

enum Program {
    Wasm { binary: Vec<u8> },
    Oci { container: String },
}

impl Program {
    fn run(&self, cmds: &[Command], env: &mut Environment) -> Vec<Output> {
        cmds.iter()
            .map(|_| Output { err: Vec::new(), ok: Vec::new(), status: Status::Success })
            .collect()
    }
}

fn get_references(node: &Node, programs: &mut HashMap<InstanceId, Entry>) {
    match node {
        Node::Commands { location, id, .. } => {
            programs.insert(
                id.to_owned(),
                Entry::Reference(match location {
                    Location::Wasm { uri } => Reference::Wasm { uri: uri.to_owned() },
                    Location::Oci { repository, image } => Reference::Oci {
                        repository: repository.to_owned(),
                        image: image.to_owned(),
                    },
                }),
            );
        }
        Node::Nodes { nodes, .. } => nodes.iter().for_each(|node| get_references(node, programs)),
    }
}
