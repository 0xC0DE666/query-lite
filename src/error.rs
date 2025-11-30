use std::result::Result as StdResult;
use thiserror::Error;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Error: Invalid Sort Direction '{0}'")]
    InvalidSortDirection(String),

    #[error("Error: Invalid Order Field '{0}'")]
    InvalidOrderField(String),

    #[error("Error: Invalid Similarity '{0}'")]
    InvalidSimilarity(String),

    #[error("Error: Invalid Parameter '{0}'")]
    InvalidParameter(String),

    #[error("Error: Invalid Search Parameters '{0}'")]
    InvalidSearchParameters(String),
}
