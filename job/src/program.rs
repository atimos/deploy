use pipeline::Section;

pub struct Programs(Vec<Program>);

impl Programs {
    pub fn new(pipeline: &Section) -> Self {
        Programs(Vec::default())
    }
}

pub struct Program {
    reference: Reference,
    bin: Option<Binary>,
}

pub enum Reference {
    Wasm(String),
    Oci(String, String),
}

pub enum Binary {
    Wasm(Vec<u8>),
    Oci(String),
}
