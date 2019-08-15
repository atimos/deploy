use super::{Command, Environment, Error, Paths, Plugin, Result};

pub struct DockerPlugin {
    pub image: String,
}

impl Plugin<'_> for DockerPlugin {
    fn run(&mut self, cmd: &Command, env: Environment, paths: Paths) -> Result {
        Result::Ok(())
    }
}
