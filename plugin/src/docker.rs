use super::{Plugin, Paths, Environment, Command, Error, Result};

pub struct DockerPlugin {
    pub image: String,
}

impl Plugin<'_> for DockerPlugin {
    fn run(&mut self, cmd: &Command, env: Environment, paths: Paths) -> Result {
        Result::Ok(())
    }
}
