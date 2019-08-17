mod prepare;

use std::collections::HashMap;
use crate::environment::Environment;
use pipeline::ExternalCommand;
use super::InstanceIds;

pub use prepare::prepare;

#[derive(Debug, Default)]
pub struct Scripts<'a>(HashMap<Vec<InstanceIds<'a>>, Script>);

#[derive(Debug)]
pub enum Script {
    Binary(Vec<u8>),
    Reference(String),
}

pub fn run(uri: &str, cmds: &[ExternalCommand], env: &mut Environment, instance_ids: InstanceIds) {
    println!("WASM: {}#{:?}", uri, cmds);
}
