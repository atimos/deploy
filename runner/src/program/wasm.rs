use crate::environment::Environment;
use pipeline::Command;

#[derive(Debug)]
pub enum Error {
    Load,
}

pub fn load(uri: &str) -> Result<Vec<u8>, Error> {
    Ok(Vec::new())
}
pub fn run(bin: &[u8], cmds: &[Command], env: Environment) -> Result<Environment, Error> {
    println!("WASM: {:?}", cmds);
    Ok(env)
}
