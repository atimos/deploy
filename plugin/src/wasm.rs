use super::{Plugin, Paths, Environment, Command, Error, Result};

pub struct WasmPlugin {
    pub bin: Vec<u8>,
}

impl Plugin<'_> for WasmPlugin {
    fn run(&mut self, cmd: &Command, env: Environment, paths: Paths) -> Result {
        Result::Ok(())
    }
}
