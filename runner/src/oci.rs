mod prepare;

use std::collections::HashMap;
use crate::environment::Environment;
use pipeline::ExternalCommand;
use super::InstanceIds;

pub use prepare::prepare;

#[derive(Debug, Default)]
pub struct Containers(HashMap<String, Container>);

#[derive(Debug)]
pub enum Container {
    Id(String),
    Image(String, String),
}

pub fn run(repo: &str, image: &str, cmds: &[ExternalCommand], env: &mut Environment, instance_ids: InstanceIds) {
    println!("OCI: docker run --rm --interactive --tty --volume $PWD:/app --user $(id -u):$(id -g) {:?}", cmds);
}
