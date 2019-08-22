mod oci;
mod wasm;

use pipeline::{Arguments, Command, InstanceId, Location, Pipeline};
use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct Programs {
    references: HashMap<Vec<InstanceId>, Reference>,
    binaries: HashMap<Vec<InstanceId>, Binary>,
}

impl Programs {
    pub fn run(&self, ids: &[InstanceId], cmds: &[Command], args: &Arguments) -> Result<(), ()> {
        if let Some(binary) = self.binaries.get(ids) {
            dbg!(binary, cmds, args);
        } else {
            if let Some(reference) = self.references.get(ids) {
                dbg!(reference, cmds, args);
            }
        }
        Ok(())
    }
}

impl TryFrom<&Pipeline> for Programs {
    type Error = ();

    fn try_from(pipeline: &Pipeline) -> Result<Self, Self::Error> {
        let mut programs = Self {
            references: HashMap::new(),
            binaries: HashMap::new(),
        };

        prepare(pipeline, Vec::new(), &mut programs);
        Ok(programs)
    }
}

#[derive(Debug)]
enum Reference {
    Wasm(String),
    Oci(String, String),
}

impl Reference {
    fn load(&self, args: Option<Arguments>) -> Result<Binary, ()> {
        Ok(match self {
            Self::Wasm(uri) => Binary::Wasm(wasm::load(&uri)?),
            Self::Oci(repository, image) => Binary::Oci(oci::load(&repository, &image)?),
        })
    }
}

#[derive(Debug)]
enum Binary {
    Wasm(Vec<u8>),
    Oci(String),
}

impl Binary {
    fn run(&self, cmds: Vec<Command>) -> Result<(), ()> {
        Ok(())
    }
}

fn prepare(pipeline: &Pipeline, mut ids: Vec<InstanceId>, programs: &mut Programs) {
    match pipeline {
        Pipeline::List {
            list, instance_id, ..
        } => {
            ids.push(instance_id.clone());
            for pipeline in list {
                prepare(pipeline, ids.clone(), programs)
            }
        }
        Pipeline::On {
            condition,
            on_success,
            on_error,
            on_abort,
            instance_id,
            ..
        } => {
            ids.push(instance_id.clone());
            prepare(condition, ids.clone(), programs);

            if let Some(pipeline) = on_success {
                prepare(pipeline, ids.clone(), programs);
            }
            if let Some(pipeline) = on_error {
                prepare(pipeline, ids.clone(), programs);
            }

            if let Some(pipeline) = on_abort {
                prepare(pipeline, ids.clone(), programs);
            }
        }
        Pipeline::Program {
            location,
            instance_id,
            ..
        } => {
            ids.push(instance_id.clone());
            match location {
                Location::Oci { repository, image } => {
                    programs
                        .references
                        .insert(ids, Reference::Oci(repository.clone(), image.clone()));
                }
                Location::Wasm { uri } => {
                    programs
                        .references
                        .insert(ids, Reference::Wasm(uri.clone()));
                }
            }
        }
    }
}
