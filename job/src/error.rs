use crate::program::Reference;
use handlebars::TemplateRenderError;

#[derive(Debug)]
pub enum Error {
    Load(Reference),
    DynamicValue(TemplateRenderError),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::DynamicValue(err) => Some(err),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Load(reference) => write!(f, "{:?}", reference),
            Self::DynamicValue(err) => write!(f, "{}", err),
        }
    }
}

impl From<TemplateRenderError> for Error {
    fn from(err: TemplateRenderError) -> Self {
        Self::DynamicValue(err)
    }
}
