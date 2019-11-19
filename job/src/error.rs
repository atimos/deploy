use crate::program::Reference;
use handlebars::TemplateRenderError;

#[derive(Debug)]
pub enum Error {
    DynamicReference(TemplateRenderError),
    StaticReference(Reference),
    Run,
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::DynamicReference(err) => Some(err),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DynamicReference(err) => std::fmt::Display::fmt(err, f),
            Self::StaticReference(reference) => {
                write!(f, "Unable to load static reference {:?}", reference)
            }
            Self::Run => write!(f, "error when running program"),
        }
    }
}

impl From<TemplateRenderError> for Error {
    fn from(err: TemplateRenderError) -> Self {
        Self::DynamicReference(err)
    }
}
