use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Embedding error: {0}")]
    EmbeddingError(String),
    #[error("Model error: {0}")]
    ModelError(String),
}
