use std::path::Path;

pub struct Environment<'a> {
    root: &'a Path,
}

impl<'a> Environment<'a> {
    pub fn new(path: &'a Path) -> Self {
        Environment { root: path }
    }
}
