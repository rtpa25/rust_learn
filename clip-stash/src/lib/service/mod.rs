pub mod action;
pub mod ask;

use crate::{ClipError, DataError};

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Clip error: {0}")]
    Clip(#[from] ClipError),
    #[error("Data error: {0}")]
    Data(DataError),
    #[error("Not found")]
    NotFound,
    #[error("Permissions not meantw: {0}")]
    PermissionError(String),
}

impl From<sqlx::Error> for ServiceError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Self::NotFound,
            other => Self::Data(DataError::DatabaseError(other)),
        }
    }
}

impl From<DataError> for ServiceError {
    fn from(value: DataError) -> Self {
        match value {
            DataError::DatabaseError(sqlx::Error::RowNotFound) => Self::NotFound,
            other => Self::Data(other),
        }
    }
}
