use ron::de::Error as RonError;

#[derive(Debug)]
pub enum Error {
    Syntax(RonError),
    NotFound(String),
    Recursion(Vec<String>),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Syntax(err) => Some(err),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
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
