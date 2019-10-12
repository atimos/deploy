use pipeline::{InstanceId, Node};
use std::collections::HashMap;

pub struct Programs(HashMap<InstanceId, (Reference, Option<Binary>)>);

impl Programs {
    pub fn new(pipeline: &Node) -> Self {
        Programs(HashMap::default())
    }
}

pub enum Reference {
    Wasm(String),
    Oci(String, String),
}

pub enum Binary {
    Wasm(Vec<u8>),
    Oci(String),
}
