use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub struct Environment {
    root: PathBuf,
    oci: Oci,
    wasm: Wasm,
    pub vars: Variables,
}

pub type Variables = HashMap<String, String>;

impl Environment {
    pub fn new(root: PathBuf) -> Self {
        Self { root, vars: Variables::new(), oci: Oci {}, wasm: Wasm {} }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn oci(&self) -> &Oci {
        &self.oci
    }

    pub fn wasm(&self) -> &Wasm {
        &self.wasm
    }
}

pub struct Oci {}

impl Oci {
    pub fn run(&self) {}
}

pub struct Wasm {}

impl Wasm {
    pub fn run(&self) {}
}
