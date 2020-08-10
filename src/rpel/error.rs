use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("error executing DB query: {0}")]
    DBQueryError(#[from] tokio_postgres::Error),
}
