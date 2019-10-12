use ron::de::Error as RonError;

#[derive(Debug)]
pub enum Error {
    Parse(ParseError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Parse(err) => Some(err),
        }
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Self::Parse(err)
    }
}

#[derive(Debug)]
pub enum ParseError {
    Syntax(RonError),
    NotFound(String),
    Recursion(Vec<String>),
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Syntax(err) => Some(err),
            _ => None,
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Syntax(err) => write!(f, "{}", err),
            Self::NotFound(uri) => write!(f, "Could not find unit \"{}\"", uri),
            Self::Recursion(name_list) => {
                write!(f, "Recursion found in {}", name_list.join(" -> "))
            }
        }
    }
}
