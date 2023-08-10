use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("Queried entity was not found in database")]
    EntityNotFound,
    #[error("Unexpected database error")]
    DbError(#[from] sqlx::error::Error),

    #[error("Error while parsing value from database")]
    ParseError,
    #[error("Unknown data store error")]
    Unknown,
}
