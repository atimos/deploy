use crate::environment::Environment;
use derivative::Derivative;
use pipeline::{Command, InstanceId};

#[derive(Derivative)]
#[derivative(Debug)]
pub enum Error {
    Load,
}

pub fn load(uri: &str) -> Result<Vec<u8>, Error> {
    Ok(Vec::new())
}
pub fn run(
    bin: &[u8],
    cmds: &[Command],
    env: Environment,
    id: &InstanceId,
) -> Result<Environment, Error> {
    println!("WASM ({:?}): {:?}", id, cmds);
    Ok(env)
}
