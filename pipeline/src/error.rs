use crate::ron::Error as RonError;
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum Error {
    Ron(RonError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ron(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Ron(err) => Some(err),
        }
    }
}

impl From<RonError> for Error {
    fn from(err: RonError) -> Self {
        Self::Ron(err)
    }
}
