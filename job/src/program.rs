use crate::{environment::Environment, error::Error};
use handlebars::Handlebars;
use pipeline::{data::Template, Arguments, Command, InstanceId, Location, Node};
use std::collections::HashMap;

pub struct Programs(HashMap<InstanceId, Program>);

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
        let mut references = Vec::new();
        get_references(pipeline, &mut references);
        Ok(Programs(
            load_programs(references).collect::<Result<HashMap<InstanceId, Program>, Error>>()?,
        ))
    }

    pub fn run(
        &self,
        id: &InstanceId,
        cmds: &[Command],
        args: &Option<Arguments>,
        env: &mut Environment,
    ) -> Result<Output, Error> {
        self.0[id].load(args, env).and_then(|binary| binary.run(cmds, args, env))
    }
}

#[derive(Debug)]
struct Program {
    reference: Reference,
    binary: Option<Binary>,
}

impl Program {
    fn load(&mut self, args: &Option<Arguments>, env: &Environment) -> Result<&Binary, Error> {
        if let None = self.binary {
            self.binary = Some(self.reference.load(args, env)?)
        }
        Ok(&self.binary.unwrap())
    }
}

#[derive(Debug)]
pub enum Reference {
    Wasm { uri: Template },
    Oci { repository: Template, image: Template },
}

impl Reference {
    fn load(&self, args: &Option<Arguments>, env: &Environment) -> Result<Binary, Error> {
        let mut hb = Handlebars::new();
        hb.set_strict_mode(true);

        match self {
            Reference::Wasm { uri } => {
                let uri = hb.render_template(uri.inner(), &None::<usize>)?;
                Ok(Binary::Wasm(Vec::new()))
            }
            Reference::Oci { repository, image } => {
                let repository = hb.render_template(repository.inner(), &None::<usize>)?;
                let image = hb.render_template(image.inner(), &None::<usize>)?;
                Ok(Binary::Oci(String::new()))
            }
        }
    }
}

#[derive(Debug)]
pub enum Binary {
    Wasm(Vec<u8>),
    Oci(String),
}
impl Binary {
    fn run(
        &self,
        cmd: &[Command],
        args: &Option<Arguments>,
        env: &mut Environment,
    ) -> Result<Output, Error> {
        Ok(Output { err: Vec::new(), ok: Vec::new(), status: Status::Success })
    }
}

fn get_references(node: &Node, references: &mut Vec<(InstanceId, Reference)>) {
    match node {
        Node::Commands { location, id, .. } => {
            references.push((id.to_owned(), match location {
                Location::Wasm { uri } => Reference::Wasm { uri: uri.to_owned() },
                Location::Oci { repository, image } => {
                    Reference::Oci { repository: repository.to_owned(), image: image.to_owned() }
                }
            }));
        }
        Node::Nodes { nodes, .. } => {
            nodes.iter().for_each(|node| get_references(node, references));
        }
    }
}

fn load_programs(
    references: Vec<(InstanceId, Reference)>,
) -> impl Iterator<Item = Result<(InstanceId, Program), Error>> {
    references.into_iter().map(|(id, reference)| Ok((id, Program { reference, binary: None })))
}
