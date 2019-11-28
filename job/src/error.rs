use crate::program::Reference;
use handlebars::TemplateRenderError;

#[derive(Debug)]
pub enum Error {
    StaticLoad(Reference),
    DynamicLoad(Reference, TemplateRenderError),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::DynamicLoad(_, err) => Some(err),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StaticLoad(reference) => write!(f, "Unable to load reference {:?}", reference),
            Self::DynamicLoad(reference, err) => {
                write!(f, "Unable to load reference {:?} - {}", reference, err)
            }
        }
    }
}
