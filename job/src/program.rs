use crate::{environment::Environment, error::Error};
use handlebars::Handlebars;
use pipeline::{data::Template, Arguments, Command, InstanceId, Location, Node};
use std::collections::HashMap;

pub struct Programs {
    inner: HashMap<InstanceId, Program>,
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
        program.load(args, env)?;
        program.run(cmds, args, env)
    }
}

#[derive(Debug)]
pub enum Program {
    Wasm { uri: Template, binary: Option<Vec<u8>> },
    Oci { repository: Template, image: Template, container: Option<String> },
}

impl Program {
    fn load(&mut self, args: &Option<Arguments>, env: &Environment) -> Result<(), Error> {
        match self {
            Self::Wasm { binary: None, uri } => {
                let uri = uri.to_owned();
                std::mem::replace(self, Self::Wasm { binary: Some(Vec::new()), uri });
                Ok(())
            }
            Self::Oci { repository, image, container: None } => {
                let image = image.to_owned();
                let repository = repository.to_owned();

                std::mem::replace(self, Self::Oci {
                    container: Some(String::new()),
                    image,
                    repository,
                });
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn run(
        &self,
        cmd: &[Command],
        args: &Option<Arguments>,
        env: &mut Environment,
    ) -> Result<Output, Error> {
        Ok(Output { err: Vec::new(), ok: Vec::new(), status: Status::Success })
    }
}

fn get_programs(node: &Node, programs: &mut HashMap<InstanceId, Program>) {
    match node {
        Node::Commands { location, id, .. } => {
            programs.insert(id.to_owned(), match location {
                Location::Wasm { uri } => Program::Wasm { uri: uri.to_owned(), binary: None },
                Location::Oci { repository, image } => Program::Oci {
                    repository: repository.to_owned(),
                    image: image.to_owned(),
                    container: None,
                },
            });
        }
        Node::Nodes { nodes, .. } => nodes.iter().for_each(|node| get_programs(node, programs)),
    }
}
