use pipeline::{Command, InstanceId, Location, Node};
use std::collections::HashMap;

pub struct Programs(HashMap<InstanceId, Program>);

impl Programs {
    pub fn new(pipeline: &Node) -> Self {
        let mut references = Vec::new();
        get_references(pipeline, &mut references);

        Programs(load_programs(references).collect())
    }

    pub fn run(
        &self,
        id: &InstanceId,
        args: &Option<pipeline::Arguments>,
        cmds: &[Command],
    ) -> Result<(), ()> {
        self.0[id].run(args, cmds)
    }
}

pub struct Program {
    reference: Reference,
    binary: Option<Binary>,
}

impl Program {
    fn run(&self, args: &Option<pipeline::Arguments>, cmds: &[Command]) -> Result<(), ()> {
        let program = self.reference.load(args)?;
        cmds.iter().map(|cmd| program.run(cmd)).collect()
    }
}

#[derive(Debug)]
pub enum Reference {
    Wasm { uri: String },
    Oci { repository: String, image: String },
}

impl Reference {
    fn load(&self, args: &Option<pipeline::Arguments>) -> Result<Binary, ()> {
        dbg!(self, args);
        match self {
            Reference::Wasm { uri } => Ok(Binary::Wasm(Vec::new())),
            Reference::Oci { repository, image } => Ok(Binary::Oci(String::new())),
        }
    }
}

#[derive(Debug)]
pub enum Binary {
    Wasm(Vec<u8>),
    Oci(String),
}

impl Binary {
    fn run(&self, cmd: &Command) -> Result<(), ()> {
        dbg!(self, cmd);
        Ok(())
    }
}

fn get_references(node: &Node, references: &mut Vec<(InstanceId, Reference)>) {
    match node {
        Node::Program { location, id, .. } => {
            references.push((id.to_owned(), match location {
                Location::Wasm { uri } => Reference::Wasm { uri: uri.to_owned() },
                Location::Oci { repository, image } => {
                    Reference::Oci { repository: repository.to_owned(), image: image.to_owned() }
                }
            }));
        }
        Node::List { list, .. } => {
            list.iter().for_each(|node| get_references(node, references));
        }
    }
}

fn load_programs(references: Vec<(InstanceId, Reference)>) -> impl Iterator<Item=(InstanceId, Program)> {
    references.into_iter().map(|(id, reference)| (id, Program { reference, binary: None }))
}
