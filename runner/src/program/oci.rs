use crate::environment::Environment;
use derivative::Derivative;
use pipeline::Command;
use std::path::Path;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum Error {
    Load,
}

pub fn load(repository: &str, image: &str, workspace: &Path) -> Result<String, Error> {
    Ok(String::new())
}

pub fn run(container_id: &str, cmds: &[Command], env: Environment) -> Result<Environment, Error> {
    println!(
        "OCI: docker run --rm --interactive --tty --volume $PWD:/app --user $(id -u):$(id -g) {:?}",
        cmds
    );
    Ok(env)
}
