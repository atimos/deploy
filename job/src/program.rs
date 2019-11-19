use crate::{environment::Environment, error::Error};
use handlebars::Handlebars;
use pipeline::{data::Template, Arguments, Command, InstanceId, Location, Node};
use std::collections::HashMap;

pub struct Programs {
    inner: HashMap<InstanceId, (Reference, Option<Program>)>,
}

pub struct Output {
    status: Status,
    err: Vec<u8>,
    ok: Vec<u8>,
}

pub enum Status {
    Error,
    Success,
    Abort,
}

impl Programs {
    pub fn load(pipeline: &Node) -> Result<Self, Error> {
        let mut programs = HashMap::new();
        get_programs(pipeline, &mut programs);
        Ok(Programs { inner: programs })
    }

    pub fn run(
        &mut self,
        id: &InstanceId,
        cmds: &[Command],
        args: &Option<Arguments>,
        env: &mut Environment,
    ) -> Result<Output, Error> {
        let program =
            self.inner.get_mut(id).ok_or_else(|| Error::UnknownInstance(id.to_owned()))?;
        Ok(Output { err: Vec::new(), ok: Vec::new(), status: Status::Success })
        // program.load(args, env)?;
        // program.run(cmds, args, env)
    }
}

#[derive(Debug)]
pub enum Reference {
    Wasm { uri: Template },
    Oci { repository: Template, image: Template },
}

impl Reference {
    fn load(self, args: &Option<Arguments>, env: &Environment) -> Result<Program, Error> {
        match self {
            Self::Wasm { uri } => Ok(Program::Wasm { binary: Vec::new() }),
            Self::Oci { repository, image } => Ok(Program::Oci { container: String::new() }),
        }
    }
}

#[derive(Debug)]
pub enum Program {
    Wasm { binary: Vec<u8> },
    Oci { container: String },
}

impl Program {
    fn run(
        &self,
        cmd: &[Command],
        args: &Option<Arguments>,
        env: &mut Environment,
    ) -> Result<Output, Error> {
        Ok(Output { err: Vec::new(), ok: Vec::new(), status: Status::Success })
    }
}

fn get_programs(node: &Node, programs: &mut HashMap<InstanceId, (Reference, Option<Program>)>) {
    match node {
        Node::Commands { location, id, .. } => {
            programs.insert(id.to_owned(), (match location {
                Location::Wasm { uri } => Reference::Wasm { uri: uri.to_owned() },
                Location::Oci { repository, image } => Reference::Oci {
                    repository: repository.to_owned(),
                    image: image.to_owned(),
                },
            }, None));
        }
        Node::Nodes { nodes, .. } => nodes.iter().for_each(|node| get_programs(node, programs)),
    }
}
