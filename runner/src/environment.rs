use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Environment {
    pub global: Option<HashMap<String, String>>,
    pub instance: Option<HashMap<String, String>>,
    pub workspace: Option<HashMap<String, String>>,
}
