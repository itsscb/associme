use std::fmt::{self, Display, Formatter};

use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug)]
pub enum ApplicationError {
    DatabaseError(sqlx::Error),
    NotFound,
    PasswordError(String),
    InvalidEmail,
    InvalidPassword,
    Unauthorized,
    InternalServerError,
}

impl IntoResponse for ApplicationError {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        let body = match &self {
            Self::DatabaseError(err) => format!("Database error: {err}"),
            Self::NotFound => "Record not found".to_string(),
            Self::PasswordError(err) => format!("Password error: {err}"),
            Self::InternalServerError => "Internal server error".to_string(),
            Self::InvalidEmail => "Invalid email".to_string(),
            Self::InvalidPassword => "Invalid password".to_string(),
            Self::Unauthorized => "Unauthorized".to_string(),
        };
        axum::http::Response::builder()
            .status(match &self {
                Self::NotFound => StatusCode::NOT_FOUND,
                Self::PasswordError(_) | Self::InvalidEmail | Self::InvalidPassword => {
                    StatusCode::BAD_REQUEST
                }
                Self::InternalServerError | Self::DatabaseError(_) => {
                    StatusCode::INTERNAL_SERVER_ERROR
                }
                Self::Unauthorized => StatusCode::UNAUTHORIZED,
            })
            .body(axum::body::Body::from(body))
            .unwrap()
    }
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::DatabaseError(err) => write!(f, "Database error: {err}"),
            Self::NotFound => write!(f, "Record not found"),
            Self::PasswordError(err) => write!(f, "Password error: {err}"),
            Self::InternalServerError => write!(f, "Internal server error"),
            Self::InvalidEmail => write!(f, "Invalid email"),
            Self::InvalidPassword => write!(f, "Invalid password"),
            Self::Unauthorized => write!(f, "Unauthorized"),
        }
    }
}

impl From<argon2::password_hash::Error> for ApplicationError {
    fn from(err: argon2::password_hash::Error) -> Self {
        Self::PasswordError(err.to_string())
    }
}

impl From<sqlx::Error> for ApplicationError {
    fn from(err: sqlx::Error) -> Self {
        // TODO: Implement this properly
        // match err {
        //     sqlx::Error::Configuration(error) => todo!(),
        //     sqlx::Error::Database(database_error) => {
        //         match database_error {
        //             sqlx::database::DatabaseError::ColumnNotFound(column) => todo!(),
        //             sqlx::database::DatabaseError::ColumnDecode { index, source } => todo!(),
        //             sqlx::database::DatabaseError::RowNotFound => todo!(),
        //             sqlx::database::DatabaseError::Serialization(error) => todo!(),
        //             sqlx::database::DatabaseError::Deserialization(error) => todo!(),
        //             sqlx::database::DatabaseError::ConstraintViolation(constraint) => todo!(),
        //             sqlx::database::DatabaseError::UniqueViolation(constraint) => match constraint {
        //                 sqlx::database::UniqueViolation::UniqueViolation { constraint, key } => todo!(),
        //                 sqlx::database::UniqueViolation::Custom(error) => todo!(),
        //             },
        //             sqlx::database::DatabaseError::ForeignKeyViolation(constraint) => todo!(),
        //             sqlx::database::DatabaseError::NotNullViolation(constraint) => todo!(),
        //             sqlx::database::DatabaseError::DbError(error) => todo!(),
        //             sqlx::database::DatabaseError::Io(error) => todo!(),
        //             sqlx::database::DatabaseError::Tls(error) => todo!(),
        //             sqlx::database::DatabaseError::Protocol(error) => todo!(),
        //             sqlx::database::DatabaseError::Configuration(error) => todo!(),
        //             sqlx::database::DatabaseError::Internal(error) => todo!(),
        //             sqlx::database::DatabaseError::Unsupported(feature) => todo!(),
        //             sqlx::database::DatabaseError::Boxed(error) => todo!(),
        //             sqlx::database::DatabaseError::Other(error) => todo!(),
        //             _ => todo!(),
        //         }

        //     },
        //     sqlx::Error::Io(error) => todo!(),
        //     sqlx::Error::Tls(error) => todo!(),
        //     sqlx::Error::Protocol(_) => todo!(),
        //     sqlx::Error::RowNotFound => todo!(),
        //     sqlx::Error::TypeNotFound { type_name } => todo!(),
        //     sqlx::Error::ColumnIndexOutOfBounds { index, len } => todo!(),
        //     sqlx::Error::ColumnNotFound(_) => todo!(),
        //     sqlx::Error::ColumnDecode { index, source } => todo!(),
        //     sqlx::Error::Encode(error) => todo!(),
        //     sqlx::Error::Decode(error) => todo!(),
        //     sqlx::Error::AnyDriverError(error) => todo!(),
        //     sqlx::Error::PoolTimedOut => todo!(),
        //     sqlx::Error::PoolClosed => todo!(),
        //     sqlx::Error::WorkerCrashed => todo!(),
        //     sqlx::Error::Migrate(migrate_error) => todo!(),
        //     _ => todo!(),
        // }
        Self::DatabaseError(err)
    }
}

// TODO: Make more specific
impl From<pasetors::errors::ClaimValidationError> for ApplicationError {
    fn from(_: pasetors::errors::ClaimValidationError) -> Self {
        Self::InternalServerError
    }
}

// TODO: Make more specific
impl From<pasetors::errors::Error> for ApplicationError {
    fn from(_: pasetors::errors::Error) -> Self {
        Self::InternalServerError
    }
}

// TODO: Make more specific
impl From<chrono::ParseError> for ApplicationError {
    fn from(_: chrono::ParseError) -> Self {
        Self::InternalServerError
    }
}

// TODO: Make more specific
impl From<chrono::OutOfRangeError> for ApplicationError {
    fn from(_: chrono::OutOfRangeError) -> Self {
        Self::InternalServerError
    }
}
