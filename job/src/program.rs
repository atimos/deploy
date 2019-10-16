use crate::{
    environment::{Arguments, Environment},
    error::Error,
    Result,
};
use handlebars::Handlebars;
use pipeline::{Command, InstanceId, Location, Node};
use std::collections::HashMap;

pub struct Programs(HashMap<InstanceId, Program>);

impl Programs {
    pub fn new(pipeline: &Node) -> Result<Self> {
        let mut references = Vec::new();
        get_references(pipeline, &mut references);

        Ok(Programs(load_programs(references).collect::<Result<HashMap<InstanceId, Program>>>()?))
    }

    pub fn run(
        &self,
        id: &InstanceId,
        cmds: &[Command],
        args: &Option<Arguments>,
        env: &mut Environment,
    ) -> Result<()> {
        dbg!(&self.0[id]).run(cmds, env, args)
    }
}

#[derive(Debug)]
pub struct Program {
    reference: Reference,
    binary: Option<Binary>,
}

impl Program {
    fn run(&self, cmds: &[Command], env: &mut Environment, args: &Option<Arguments>) -> Result<()> {
        if let Some(bin) = &self.binary {
            cmds.iter().map(|cmd| bin.run(cmd, env)).collect()
        } else {
            let bin = self.reference.load(args)?;
            cmds.iter().map(|cmd| bin.run(cmd, env)).collect()
        }
    }
}

#[derive(Debug)]
pub enum Reference {
    Wasm { uri: String },
    Oci { repository: String, image: String },
}

impl Reference {
    fn load(&self, args: &Option<Arguments>) -> Result<Binary> {
        let mut hb = Handlebars::new();
        hb.set_strict_mode(true);

        match self {
            Reference::Wasm { uri } => {
                let uri = hb.render_template(uri, args)?;
                Ok(Binary::Wasm(Vec::new()))
            }
            Reference::Oci { repository, image } => {
                let repository = hb.render_template(repository, args)?;
                let image = hb.render_template(image, args)?;
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
    fn run(&self, cmd: &Command, env: &mut Environment) -> Result<()> {
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

fn load_programs(
    references: Vec<(InstanceId, Reference)>,
) -> impl Iterator<Item = Result<(InstanceId, Program)>> {
    references.into_iter().map(|(id, reference)| match reference.load(&None) {
        Ok(bin) => Ok((id, Program { reference, binary: Some(bin) })),
        Err(Error::DynamicValue(_)) => Ok((id, Program { reference, binary: None })),
        Err(err) => Err(err),
    })
}
