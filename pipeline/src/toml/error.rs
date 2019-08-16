use toml::de::Error as TomlError;

#[derive(Debug)]
pub enum Error {
    Parse(TomlError),
    CommandMix,
    NoCommandFound,
    InvalidArgumentsType(&'static str, &'static str),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(err) => write!(f, "{}", err),
            Self::CommandMix => write!(f, "command and commands is not allowed to be used at the same time"),
            Self::NoCommandFound => write!(f, "command or commands is missing"),
            Self::InvalidArgumentsType(expected, actual) => write!(
                f,
                "Type \"{}\" of arguments is not valid, expected \"{}\"",
                actual,
                expected,
            ),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Parse(err) => Some(err),
            _ => None,
        }
    }
}

impl From<TomlError> for Error {
    fn from(err: TomlError) -> Self {
        Self::Parse(err)
    }
}
