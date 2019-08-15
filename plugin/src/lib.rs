mod docker;
mod wasm;
mod https;

use std::{path::Path, collections::HashMap};
use pipeline::Command;

pub use docker::DockerPlugin;
pub use wasm::WasmPlugin;
pub use https::HttpsPlugin;

pub type Result = std::result::Result<(), Error>;

#[derive(Debug)]
pub struct Environment {
    pub global: Option<HashMap<String, String>>,
    pub domain: Option<HashMap<String, String>>,
    pub instance: Option<HashMap<String, String>>,
}

#[derive(Debug)]
pub struct Paths<'a> {
    pub work: Option<&'a Path>,
    pub publish: Option<&'a Path>,
    pub artifact: Option<&'a Path>,
    pub scratch: Option<&'a Path>,
}

#[derive(Debug)]
pub enum Error {
    Custom(String),
}

pub trait Plugin<'a>: std::fmt::Debug {
    fn run(&mut self, cmd: &Command, env: Environment, paths: Paths<'a>) -> Result;
}
