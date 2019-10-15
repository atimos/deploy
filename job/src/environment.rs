use std::path::PathBuf;

pub struct Environment {
    root: PathBuf,
}

impl Environment {
    pub fn new(path: PathBuf) -> Self {
        Environment { root: path }
    }
}
