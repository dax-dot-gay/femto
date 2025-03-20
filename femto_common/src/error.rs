#[derive(thiserror::Error, Clone, Debug)]
pub enum Error {
    #[error("Invalid template source specfier: {0}")]
    ProjectSource(String),
}

pub type FResult<T> = Result<T, Error>;
