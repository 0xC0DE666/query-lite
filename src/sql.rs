#[cfg(feature = "sql")]
pub const NULL: &str = "null";

#[cfg(feature = "sql")]
pub use rusqlite::types::{ToSql, Value};
