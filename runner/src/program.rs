use std::collections::HashMap;
use super::{oci::Container, wasm::Script};
use pipeline::{InstanceId, Pipeline, ControlStructure, Command};

type InstanceIds = Vec<InstanceId>;
type ProgramMap = HashMap<InstanceIds, Program>;

#[derive(Debug)]
pub struct Programs {
    inner: ProgramMap,
}

impl<'a> From<&'a Pipeline> for Programs {
    fn from(pipeline: &Pipeline) -> Self {
        let mut programs = ProgramMap::new();

        for step in &pipeline.steps {
            load_ctrl(&step.run, InstanceIds::new(), &mut programs);
        }

        Self {
            inner: programs
        }
    }
}

#[derive(Debug)]
pub enum Program {
    Wasm(Script),
    Oci(Container),
}

fn load_ctrl<'a>(ctrl: &'a ControlStructure, mut instance_ids: InstanceIds, programs: &mut ProgramMap) {
    match ctrl {
        ControlStructure::Command { run, instance_id } => {
            instance_ids.push(instance_id.clone());
            load_cmd(run, instance_ids, programs)
        }
        _ => {}
    }
}

fn load_cmd<'a>(cmd: &'a Command, instance_ids: InstanceIds, programs: &mut ProgramMap) {
    match cmd {
        Command::Oci { repository, image, .. } => {
            programs.insert(instance_ids, Program::Oci(Container::Image(repository.to_owned(), image.to_owned())));
        }
        Command::Wasm { uri, .. } => {
            programs.insert(instance_ids, Program::Wasm(Script::Reference(uri.to_owned())));
        }
    }
}
