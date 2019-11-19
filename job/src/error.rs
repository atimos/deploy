use pipeline::InstanceId;
use crate::program::Program;
use handlebars::TemplateRenderError;

#[derive(Debug)]
pub enum Error {
    UnknownInstance(InstanceId),
    DynamicLoad(TemplateRenderError),
    StaticLoad(Program),
    Run,
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::DynamicLoad(err) => Some(err),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DynamicLoad(err) => std::fmt::Display::fmt(err, f),
            Self::StaticLoad(reference) => {
                write!(f, "Unable to load static reference {:?}", reference)
            }
            Self::UnknownInstance(instance) => write!(f, "program with id {} not found", instance),
            Self::Run => write!(f, "error when running program"),
        }
    }
}

impl From<TemplateRenderError> for Error {
    fn from(err: TemplateRenderError) -> Self {
        Self::DynamicLoad(err)
    }
}
