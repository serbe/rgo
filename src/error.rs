// pub type Result<T> = std::result::Result<T, ServiceError>;

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Socket address parse: {0}")]
    AddrParse(#[from] std::net::AddrParseError),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("IO: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Rpel: {0}")]
    Rpel(#[from] rpel::error::RpelError),
    #[error("Serde JSON: {0}")]
    SJError(#[from] serde_json::error::Error),
    #[error("Not auth")]
    NotAuth,
    #[error("Not permission")]
    NotPermission,
    #[error("Hyper: {0}")]
    Hyper(#[from] hyper::Error),
    #[error("Hyper http: {0}")]
    HyperHttp(#[from] hyper::http::Error),
    #[error("Request not contain state")]
    NoState,
    #[error("No build router")]
    Router,
    #[error["std {0}"]]
    Std(#[from] Box<dyn std::error::Error + Send + Sync>), // #[error("error executing DB query: {0}")]
                                                           // DBQueryError(#[from] tokio_postgres::Error)
}

// impl warp::reject::Reject for ServiceError {}

// impl From<ServiceError> for warp::Rejection {
//     fn from(error: ServiceError) -> warp::Rejection {
//         warp::reject::custom(error)
//     }
// }

// impl From<ServiceError> for hyper::Response<()> {
//     fn from(error: ServiceError) -> hyper::Response<()> {
//         match error {
//             ServiceError::AddrParse(_) => {}
//             ServiceError::BadRequest(_) => {}
//             ServiceError::IOError(_) => {}
//             ServiceError::Rpel(_) => {}
//             ServiceError::SJError(_) => {}
//             ServiceError::NotAuth => {}
//             ServiceError::NotPermission => {}
//             ServiceError::Hyper(_) => {}
//             ServiceError::HyperHttp(_) => {}
//             ServiceError::NoState => {}
//             ServiceError::Router => {}
//         }
//         hyper::Response::builder().status(200).body(()).unwrap()
//     }
// }
