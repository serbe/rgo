pub type Result<T> = std::result::Result<T, ServiceError>;

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Socket address parse: {0}")]
    AddrParse(#[from] std::net::AddrParseError),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Deadpool error: {0}")]
    PoolError(#[from] deadpool_postgres::PoolError),
    #[error("Serde JSON error: {0}")]
    SjError(#[from] serde_json::error::Error),
    #[error("Not auth")]
    NotAuth,
    #[error("Not permission")]
    NotPermission,
    #[error("Executing DB query: {0}")]
    DbQuery(#[from] tokio_postgres::Error),
    #[error("Empty DB name")]
    EmptyDbName,
    #[error("Hyper: {0}")]
    Hyper(#[from] hyper::Error),
    #[error("Create router: {0}")]
    Router(String),
    #[error("No get pool from state")]
    NoPool,
    #[error("No get users from state")]
    NoUsers,
}

// impl warp::reject::Reject for ServiceError {}

// impl From<ServiceError> for warp::Rejection {
//     fn from(error: ServiceError) -> warp::Rejection {
//         warp::reject::custom(error)
//     }
// }
