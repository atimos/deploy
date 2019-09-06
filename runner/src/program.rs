mod error;
mod oci;
mod wasm;

use crate::{environment::Environment, template::render};
use derivative::Derivative;
use pipeline::{Arguments, Command, Id, Location, Pipeline};
use std::{collections::HashMap, convert::TryFrom, path::Path};

pub use error::Error;

#[derive(Derivative)]
#[derivative(Debug, Default)]
pub struct Programs {
    references: HashMap<Vec<Id>, Reference>,
    binaries: HashMap<Vec<Id>, Binary>,
}

impl Programs {
    pub fn run(
        &self,
        id: &[Id],
        cmds: &[Command],
        args: &Option<Arguments>,
        env: Environment,
    ) -> Result<Environment, Error> {
        if let Some(binary) = self.binaries.get(id) {
            return binary.run(cmds, env);
        }
        if let Some(reference) = self.references.get(id) {
            return reference.load(args, &env.workspace)?.run(cmds, env);
        }
        Ok(env)
    }
}

type References = Vec<(Vec<Id>, Reference)>;

impl TryFrom<(&Pipeline, &Path)> for Programs {
    type Error = Error;

    fn try_from((pipeline, workspace): (&Pipeline, &Path)) -> Result<Self, Self::Error> {
        let mut references = References::new();
        prepare(pipeline, Vec::new(), &mut references);
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
    fn run(&self, cmds: &[Command], env: Environment) -> Result<Environment, Error> {
        Ok(match self {
            Self::Wasm(bin) => wasm::run(bin, &cmds, env)?,
            Self::Oci(container) => oci::run(container, &cmds, env)?,
        })
    }
}

fn prepare(pipeline: &Pipeline, mut ids: Vec<Id>, references: &mut References) {
    match pipeline {
        Pipeline::List { list, id, .. } => {
            ids.push(id.clone());
            for pipeline in list {
                prepare(pipeline, ids.clone(), references)
            }
        }
        Pipeline::On { cond, success, error, abort, id, .. } => {
            ids.push(id.clone());
            prepare(cond, ids.clone(), references);

            if let Some(pipeline) = success {
                prepare(pipeline, ids.clone(), references);
            }
            if let Some(pipeline) = error {
                prepare(pipeline, ids.clone(), references);
            }

            if let Some(pipeline) = abort {
                prepare(pipeline, ids.clone(), references);
            }
        }
        Pipeline::Program { location, id, .. } => {
            ids.push(id.clone());
            references.push(match location {
                Location::Oci { repository, image } => {
                    (ids, Reference::Oci(repository.clone(), image.clone()))
                }
                Location::Wasm { uri } => (ids, Reference::Wasm(uri.clone())),
            });
        }
    }
}
