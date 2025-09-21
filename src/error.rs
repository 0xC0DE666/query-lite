use std::result::Result as StdResult;
use thiserror::Error;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error: Invalid Sort Order '{0}'")]
    InvalidSortOrder(String),

    #[error("Error: Invalid Sort Field '{0}'")]
    InvalidSortField(String),

    #[error("Error: Invalid Similaritty '{0}'")]
    InvalidSimilaritty(String),

    #[error("Error: Invalid Parameter '{0}'")]
    InvalidParameter(String),

    #[error("Error: Invalid Search Parameters '{0}'")]
    InvalidSearchParameters(String),
}
