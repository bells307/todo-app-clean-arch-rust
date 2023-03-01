use thiserror::Error;

#[derive(Debug, Error)]
pub enum TodoError {
    #[error("todo name is empty")]
    NameIsEmpty,
    #[error("todo {0} not found")]
    NotFound(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
