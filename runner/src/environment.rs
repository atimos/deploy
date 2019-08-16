use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Environment {
    pub status: Status,
    pub config: Option<HashMap<String, String>>,
    pub unit: Option<HashMap<String, String>>,
    pub instance: Option<HashMap<String, String>>,
    pub workspace: PathBuf,
}

#[derive(Debug, Clone)]
pub enum Status {
    Error,
    Success,
    Abort,
}
