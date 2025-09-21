use std::result::Result as StdResult;
use thiserror::Error;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error: Invalid Order '{0}'")]
    InvalidOrder(String),

    #[error("Error: Invalid Similaritty '{0}'")]
    InvalidSimilaritty(String),
}
