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

    pub fn run(&self, id: &InstanceId, args: &Arguments, cmds: Commands) -> Result<(), ()> {
        let program = &self.0[id];
        load(&program.0, args).map(drop)?;
        dbg!(cmds);
        Ok(())
    }
}

#[derive(Debug)]
pub enum Reference {
    Wasm { uri: String },
    Oci { repository: String, image: String },
}

pub enum Binary {
    Wasm(Vec<u8>),
    Oci(String),
}

fn get_programs(node: &Node, references: &mut Vec<(InstanceId, Program)>) {
    match node {
        Node::Program { location, id, .. } => {
            references.push((
                id.to_owned(),
                (
                    match location {
                        Location::Wasm { uri } => Reference::Wasm { uri: uri.to_owned() },
                        Location::Oci { repository, image } => Reference::Oci {
                            repository: repository.to_owned(),
                            image: image.to_owned(),
                        },
                    },
                    None,
                ),
            ));
        }
        Node::List { list, .. } => {
            list.iter().for_each(|node| get_programs(node, references));
        }
    }
}

fn load(reference: &Reference, args: &Arguments) -> Result<Binary, ()> {
    dbg!(reference, args);
    match reference {
        Reference::Wasm { uri } => Ok(Binary::Wasm(Vec::new())),
        Reference::Oci { repository, image } => Ok(Binary::Oci(String::new())),
    }
}
