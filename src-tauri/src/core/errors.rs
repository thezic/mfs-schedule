use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("Queried entity was not found in database")]
    EntityNotFound,
    #[error("Unexpected database error")]
    DbError(#[from] sqlx::error::Error),

    #[error("Error while parsing value `{value:?}` from database: {error:?}")]
    ParseError { value: String, error: String },
    #[error("Unknown data store error")]
    Unknown,
}
