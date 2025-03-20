#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid template source specfier: {0}")]
    ProjectSource(String),

    #[error("Encountered an IO error: {0:?}")]
    IO(std::io::Error),

    #[error("Encountered a serialization error: {0}")]
    Serialization(String)
}

pub type FResult<T> = Result<T, Error>;

impl Error {
    pub fn project_source(source: impl Into<String>) -> Self {
        Self::ProjectSource(source.into())
    }

    pub fn io(error: std::io::Error) -> Self {
        Self::IO(error)
    }

    pub fn serialization(error: impl serde::ser::Error) -> Self {
        Self::Serialization(error.to_string())
    }
}
