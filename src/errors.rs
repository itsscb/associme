use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;
#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Missing Data: {0}")]
    MissingData(String),
    #[error("Record not found")]
    NotFound,
    #[error("Password error: {0}")]
    PasswordError(String),
    #[error("Invalid email")]
    InvalidEmail,
    #[error("Invalid password")]
    InvalidPassword,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Internal server error")]
    InternalServerError(#[from] Box<dyn std::error::Error + Send + Sync>),
    // #[error("Argon2 error: {0}")]
    // Argon2Error(#[from] argon2::password_hash::Error),
    // #[error("SQLx error: {0}")]
    // SqlxError(#[from] sqlx::Error),
    // #[error("Pasetors claim validation error: {0}")]
    // PasetorsClaimValidationError(#[from] pasetors::errors::ClaimValidationError),
    // #[error("Pasetors error: {0}")]
    // PasetorsError(#[from] pasetors::errors::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

impl IntoResponse for ApplicationError {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        let body = match &self {
            Self::MissingData(err) => format!("Missing data: {err}"),
            Self::DatabaseError(err) => format!("Database error: {err}"),
            Self::NotFound => "Record not found".to_string(),
            Self::PasswordError(err) => format!("Password error: {err}"),
            Self::InternalServerError(_) => "Internal server error".to_string(),
            Self::InvalidEmail => "Invalid email".to_string(),
            Self::InvalidPassword => "Invalid password".to_string(),
            Self::Unauthorized => "Unauthorized".to_string(),
            Self::SerializationError(err) => format!("Serialization error: {err}"),
        };
        axum::http::Response::builder()
            .status(match &self {
                Self::NotFound => StatusCode::NOT_FOUND,
                Self::PasswordError(_) | Self::InvalidEmail | Self::InvalidPassword => {
                    StatusCode::BAD_REQUEST
                }
                Self::Unauthorized => StatusCode::UNAUTHORIZED,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            })
            .body(axum::body::Body::from(body))
            .unwrap()
    }
}

impl From<argon2::password_hash::Error> for ApplicationError {
    fn from(err: argon2::password_hash::Error) -> Self {
        Self::PasswordError(err.to_string())
    }
}

// impl From<sqlx::Error> for ApplicationError {
//     fn from(err: sqlx::Error) -> Self {
//         // TODO: Implement this properly
//         // match err {
//         //     sqlx::Error::Configuration(error) => todo!(),
//         //     sqlx::Error::Database(database_error) => {
//         //         match database_error {
//         //             sqlx::database::DatabaseError::ColumnNotFound(column) => todo!(),
//         //             sqlx::database::DatabaseError::ColumnDecode { index, source } => todo!(),
//         //             sqlx::database::DatabaseError::RowNotFound => todo!(),
//         //             sqlx::database::DatabaseError::Serialization(error) => todo!(),
//         //             sqlx::database::DatabaseError::Deserialization(error) => todo!(),
//         //             sqlx::database::DatabaseError::ConstraintViolation(constraint) => todo!(),
//         //             sqlx::database::DatabaseError::UniqueViolation(constraint) => match constraint {
//         //                 sqlx::database::UniqueViolation::UniqueViolation { constraint, key } => todo!(),
//         //                 sqlx::database::UniqueViolation::Custom(error) => todo!(),
//         //             },
//         //             sqlx::database::DatabaseError::ForeignKeyViolation(constraint) => todo!(),
//         //             sqlx::database::DatabaseError::NotNullViolation(constraint) => todo!(),
//         //             sqlx::database::DatabaseError::DbError(error) => todo!(),
//         //             sqlx::database::DatabaseError::Io(error) => todo!(),
//         //             sqlx::database::DatabaseError::Tls(error) => todo!(),
//         //             sqlx::database::DatabaseError::Protocol(error) => todo!(),
//         //             sqlx::database::DatabaseError::Configuration(error) => todo!(),
//         //             sqlx::database::DatabaseError::Internal(error) => todo!(),
//         //             sqlx::database::DatabaseError::Unsupported(feature) => todo!(),
//         //             sqlx::database::DatabaseError::Boxed(error) => todo!(),
//         //             sqlx::database::DatabaseError::Other(error) => todo!(),
//         //             _ => todo!(),
//         //         }

//         //     },
//         //     sqlx::Error::Io(error) => todo!(),
//         //     sqlx::Error::Tls(error) => todo!(),
//         //     sqlx::Error::Protocol(_) => todo!(),
//         //     sqlx::Error::RowNotFound => todo!(),
//         //     sqlx::Error::TypeNotFound { type_name } => todo!(),
//         //     sqlx::Error::ColumnIndexOutOfBounds { index, len } => todo!(),
//         //     sqlx::Error::ColumnNotFound(_) => todo!(),
//         //     sqlx::Error::ColumnDecode { index, source } => todo!(),
//         //     sqlx::Error::Encode(error) => todo!(),
//         //     sqlx::Error::Decode(error) => todo!(),
//         //     sqlx::Error::AnyDriverError(error) => todo!(),
//         //     sqlx::Error::PoolTimedOut => todo!(),
//         //     sqlx::Error::PoolClosed => todo!(),
//         //     sqlx::Error::WorkerCrashed => todo!(),
//         //     sqlx::Error::Migrate(migrate_error) => todo!(),
//         //     _ => todo!(),
//         // }
//         Self::DatabaseError(err)
//     }
// }

// TODO: Make more specific
impl From<pasetors::errors::ClaimValidationError> for ApplicationError {
    fn from(_: pasetors::errors::ClaimValidationError) -> Self {
        Self::InternalServerError(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Claim validation error",
        )))
    }
}

// TODO: Make more specific
impl From<pasetors::errors::Error> for ApplicationError {
    fn from(_: pasetors::errors::Error) -> Self {
        Self::InternalServerError(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Paseto Token error",
        )))
    }
}

// // TODO: Make more specific
// impl From<chrono::ParseError> for ApplicationError {
//     fn from(_: chrono::ParseError) -> Self {
//         Self::InternalServerError
//     }
// }

// // TODO: Make more specific
// impl From<chrono::OutOfRangeError> for ApplicationError {
//     fn from(_: chrono::OutOfRangeError) -> Self {
//         Self::InternalServerError
//     }
// }
