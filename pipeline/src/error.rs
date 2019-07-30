use toml::de::Error as TomlError;

use super::pipeline::TaskId;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Toml(TomlError),
    TaskNotFound(TaskId),
    TaskRecursion(Vec<TaskId>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Toml(err) => write!(f, "{}", err),
            Self::TaskNotFound(task) => write!(f, "Could not find task \"{}\" in pipeline", task),
            Self::TaskRecursion(tasks) => write!(
                f,
                r#"Recursion found in pipeline between tasks "{}""#,
                tasks.join(r#"", ""#)
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
