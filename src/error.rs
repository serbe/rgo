pub type Result<T> = std::result::Result<T, ServiceError>;

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Socket address parse: {0}")]
    AddrParse(#[from] std::net::AddrParseError),
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Deadpool error: {0}")]
    PoolError(#[from] deadpool_postgres::PoolError),

    #[error("Serde JSON error: {0}")]
    SJError(#[from] serde_json::error::Error),
    #[error("Not auth")]
    NotAuth,
    #[error("Not permission")]
    NotPermission,
    // #[error("Error get client")]
    // ClientGet,
    #[error("error executing DB query: {0}")]
    DBQueryError(#[from] tokio_postgres::Error),
}

impl warp::reject::Reject for ServiceError {

}

// impl ResponseError for ServiceError {
//     fn error_response(&self) -> HttpResponse {
//         match self {
//             // ServiceError::InternalServerError => HttpResponse::BadRequest()
//             //     .reason("Internal server error. Please try again later")
//             //     .finish(),
//             ServiceError::BadRequest(_) => {
//                 HttpResponse::BadRequest().reason("bad request").finish()
//             }
//             // ServiceError::IOError(_) => HttpResponse::BadRequest().reason("io error").finish(),
//             ServiceError::PoolError(_) => HttpResponse::BadRequest()
//                 .reason("unable to connect to the database")
//                 .finish(),
//             ServiceError::DBQueryError(_) => HttpResponse::BadRequest().reason("db error").finish(),
//             ServiceError::SJError(_) => HttpResponse::BadRequest()
//                 .reason("serde json error")
//                 .finish(),
//             ServiceError::NotAuth => HttpResponse::NotFound().finish(),
//             // ServiceError::FailedAuth => HttpResponse::BadRequest()
//             //     .reason("Internal server error. Please try again later")
//             //     .finish(),
//             // ServiceError::ClientGet => HttpResponse::BadRequest()
//             //     .reason("Internal server error. Please try again later")
//             //     .finish(),
//             ServiceError::NotPermission => HttpResponse::BadRequest()
//                 .reason("Internal server error. Please try again later")
//                 .finish(),
//         }
//     }
// }
