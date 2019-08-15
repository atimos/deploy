use toml::de::Error as TomlError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Toml(TomlError),
    UnitNotFound(String),
    UnitRecursion(Vec<String>),
    ArgumentMissing(String),
    ArgumentsMissing,
    UnexpectedArgument(String),
    UnexpectedArguments,
    InvalidArgumentsType(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Toml(err) => write!(f, "{}", err),
            Self::UnitNotFound(uri) => write!(f, "Could not find unit \"{}\"", uri),
            Self::UnitRecursion(name_list) => {
                write!(f, "Recursion found in {}", name_list.join(" -> "))
            }
            Self::ArgumentMissing(arg) => write!(f, "Argument \"{}\" is missing", arg),
            Self::ArgumentsMissing => write!(f, "Unit requires arguments"),
            Self::UnexpectedArgument(arg) => write!(f, "Unexpected argument \"{}\"", arg),
            Self::UnexpectedArguments => write!(f, "Unit does not expect any arguments"),
            Self::InvalidArgumentsType(expected) => write!(
                f,
                "Type of arguments is not valid, expected \"{}\"",
                expected
            ),
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
