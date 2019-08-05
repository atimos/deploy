use toml::de::Error as TomlError;

use super::Url;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Toml(TomlError),
    DomainMissing(Url),
    UnitNotFound(Url),
    UnitRecursion(Vec<Url>),
    ArgumentMissing(String),
    UnexpectedArgument(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Toml(err) => write!(f, "{}", err),
            Self::DomainMissing(uri) => write!(f, "Command uri \"{}\" is missing domain ", uri),
            Self::UnitNotFound(uri) => write!(f, "Could not find unit \"{}\"", uri),
            Self::UnitRecursion(uri_list) => write!(
                f,
                "Recursion found between units {}",
                uri_list
                    .iter()
                    .map(Url::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::ArgumentMissing(arg) => write!(f, "Argument \"{}\" is missing", arg),
            Self::UnexpectedArgument(arg) => write!(f, "Unexpected argument \"{}\"", arg),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Toml(err) => Some(err),
            _ => None,
        }
    }
}

impl From<TomlError> for Error {
    fn from(err: TomlError) -> Self {
        Self::Toml(err)
    }
}
