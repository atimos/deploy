use crate::program::Reference;

#[derive(Debug)]
pub enum Error {
    Load(Reference)
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Load(reference) => write!(f, "{:?}", reference),
        }
    }
}
