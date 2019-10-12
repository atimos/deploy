use pipeline::{Command, InstanceId, Location, Node};
use std::collections::HashMap;

pub type Arguments = Option<pipeline::Arguments>;
pub type Commands<'a> = &'a [Command];
pub type Program = (Reference, Option<Binary>);
pub struct Programs(HashMap<InstanceId, Program>);

impl Programs {
    pub fn new(pipeline: &Node) -> Self {
        let mut references = Vec::new();
        get_programs(pipeline, &mut references);

        Programs(references.into_iter().collect())
    }

    pub fn run(&self, id: &InstanceId, ccmds: Commands, args: &Arguments) -> Result<(), ()> {
        let program = &self.0[id];
        Ok(())
    }
}

pub enum Reference {
    Wasm(String),
    Oci(String, String),
}

pub enum Binary {
    Wasm(Vec<u8>),
    Oci(String),
}

fn get_programs(node: &Node, references: &mut Vec<(InstanceId, Program)>) {
    match node {
        Node::Program { location, id, .. } => {
            references.push((id.to_owned(), match location {
                Location::Wasm { uri } => (Reference::Wasm(uri.to_owned()), None),
                Location::Oci { repository, image } => {
                    (Reference::Oci(repository.to_owned(), image.to_owned()), None)
                }
            }));
        }
        Node::List { list, .. } => {
            list.iter().for_each(|node| get_programs(node, references));
        }
        Node::On { condition, success, error, abort, .. } => {
            get_programs(condition, references);
            success.as_ref().map(|node| get_programs(&node, references));
            error.as_ref().map(|node| get_programs(&node, references));
            abort.as_ref().map(|node| get_programs(&node, references));
        }
    }
}

fn load(referenec: &Reference, args: &Arguments) -> Result<Binary, ()> {
    Err(())
}
