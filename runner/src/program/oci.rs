use crate::environment::Environment;
use derivative::Derivative;
use pipeline::{Command, InstanceId};
use std::path::Path;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum Error {
    Load,
}

pub fn load(repository: &str, image: &str, workspace: &Path) -> Result<String, Error> {
    Ok(String::new())
}

pub fn run(
    container_id: &str,
    cmds: &[Command],
    env: Environment,
    id: &InstanceId,
) -> Result<Environment, Error> {
    println!(
        "OCI ({:?}): docker run --rm --interactive --tty --volume $PWD:/app --user $(id -u):$(id -g) {:?}",
        id,
        cmds
    );
    Ok(env)
}
