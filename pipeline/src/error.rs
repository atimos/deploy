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
    Convertion(ConvertionError)
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
            Self::Convertion(err) => write!(f, "{}", err)
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Toml(err) => Some(err),
            Self::Convertion(err) => Some(err),
            _ => None,
        }
    }
}

impl From<TomlError> for Error {
    fn from(err: TomlError) -> Self {
        Self::Toml(err)
    }
}

impl From<ConvertionError> for Error {
    fn from(err: ConvertionError) -> Self {
        Self::Convertion(err)
    }
}

#[derive(Debug)]
pub enum ConvertionError {
    InvalidArgumentsType(&'static str, &'static str),
}
impl std::error::Error for ConvertionError {
}

impl std::fmt::Display for ConvertionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidArgumentsType(expected, actual) => write!(
                f,
                "Type \"{}\" of arguments is not valid, expected \"{}\"",
                actual,
                expected,
            ),
        }
    }
}
