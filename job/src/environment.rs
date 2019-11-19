use std::{collections::HashMap, path::PathBuf};

pub struct Environment {
    root: PathBuf,
    pub global: HashMap<String, String>,
    pub local: HashMap<String, String>,
}

impl Environment {
    pub fn new(root: PathBuf) -> Self {
        Self { root, global: HashMap::new(), local: HashMap::new() }
    }
}
