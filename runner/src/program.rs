mod error;
mod oci;
mod wasm;

use crate::{environment::Environment, template::render};
use derivative::Derivative;
use pipeline::{Arguments, Block, Command, InstanceId, Location};
use std::{collections::HashMap, convert::TryFrom, path::Path};

pub use error::Error;

type References = Vec<(InstanceId, Reference)>;

#[derive(Derivative)]
#[derivative(Debug, Default)]
pub struct Programs {
    references: HashMap<InstanceId, Reference>,
    binaries: HashMap<InstanceId, Binary>,
}

impl Programs {
    pub fn run(
        &self,
        id: &InstanceId,
        cmds: &[Command],
        args: &Option<Arguments>,
        env: Environment,
    ) -> Result<Environment, Error> {
        if let Some(binary) = self.binaries.get(id) {
            return binary.run(cmds, env, id);
        }
        if let Some(reference) = self.references.get(id) {
            return reference.load(args, &env.workspace)?.run(cmds, env, id);
        }
        Ok(env)
    }
}

impl TryFrom<(&Block, &Path)> for Programs {
    type Error = Error;

    fn try_from((pipeline, workspace): (&Block, &Path)) -> Result<Self, Self::Error> {
        let mut references = References::new();
        get(pipeline, &mut references);
        let mut programs = Self::default();

        for (id, reference) in references {
            match reference.load(&None, workspace) {
                Ok(bin) => {
                    programs.binaries.insert(id, bin);
                }
                Err(Error::Template(_)) => {
                    programs.references.insert(id, reference);
                }
                Err(err) => return Err(err),
            }
        }

        Ok(programs)
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
enum Reference {
    Wasm(String),
    Oci(String, String),
}

impl Reference {
    fn load(&self, args: &Option<Arguments>, workspace: &Path) -> Result<Binary, Error> {
        Ok(match self {
            Self::Wasm(uri) => Binary::Wasm(wasm::load(&render(&uri, &args)?)?),
            Self::Oci(repository, image) => Binary::Oci(oci::load(
                &render(&repository, &args)?,
                &render(&image, &args)?,
                workspace,
            )?),
        })
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
enum Binary {
    Wasm(Vec<u8>),
    Oci(String),
}

impl Binary {
    fn run(
        &self,
        cmds: &[Command],
        env: Environment,
        id: &InstanceId,
    ) -> Result<Environment, Error> {
        Ok(match self {
            Self::Wasm(bin) => wasm::run(bin, &cmds, env, id)?,
            Self::Oci(container) => oci::run(container, &cmds, env, id)?,
        })
    }
}

fn get(pipeline: &Block, references: &mut References) {
    match pipeline {
        Block::List { list, .. } => {
            for pipeline in list {
                get(pipeline, references)
            }
        }
        Block::On { condition, success, error, abort, .. } => {
            get(condition, references);

            if let Some(block) = success {
                get(block, references);
            }
            if let Some(block) = error {
                get(block, references);
            }

            if let Some(block) = abort {
                get(block, references);
            }
        }
        Block::Commands { location, id, .. } => {
            references.push(match location {
                Location::Oci { repository, image } => {
                    (id.clone(), Reference::Oci(repository.clone(), image.clone()))
                }
                Location::Wasm { uri } => (id.clone(), Reference::Wasm(uri.clone())),
            });
        }
    }
}
