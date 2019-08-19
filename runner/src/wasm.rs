use crate::environment::Environment;
use pipeline::ExternalCommand;

#[derive(Debug)]
pub enum Script {
    Binary(Vec<u8>),
    Reference(String),
}

impl Script {
    pub fn run(&self, cmds: &[ExternalCommand], env: &mut Environment) {
        println!("WASM: {:?}", cmds);
    }
}
