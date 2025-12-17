#[cfg(feature = "sqlx")]
use std::borrow::Cow;

pub const NULL: &str = "null";

// Re-export database traits for convenience
#[cfg(feature = "rusqlite")]
pub use rusqlite::types::ToSql;

#[cfg(feature = "sqlx")]
pub use sqlx::{Encode, Type};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// The value is a `NULL` value.
    Null,
    /// The value is a signed integer.
    Integer(i64),
    /// The value is a floating point number.
    Real(f64),
    /// The value is a text string.
    Text(String),
    /// The value is a blob of data
    Blob(Vec<u8>),
}

// RUSQLITE
#[cfg(feature = "rusqlite")]
impl rusqlite::types::ToSql for Value {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        let val = match self {
            Value::Null => rusqlite::types::ValueRef::Null,
            Value::Integer(i) => rusqlite::types::ValueRef::Integer(*i),
            Value::Real(r) => rusqlite::types::ValueRef::Real(*r),
            Value::Text(t) => rusqlite::types::ValueRef::Text(t.as_bytes()),
            Value::Blob(b) => rusqlite::types::ValueRef::Blob(b),
        };

        Ok(rusqlite::types::ToSqlOutput::Borrowed(val))
    }
}

// SQLX
#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Sqlite> for Value {
    fn type_info() -> <sqlx::Sqlite as sqlx::Database>::TypeInfo {
        // Value can represent any SQLite type, so we return TEXT as a common denominator.
        // The actual type will be determined at runtime during encoding.
        <String as sqlx::Type<sqlx::Sqlite>>::type_info()
    }

    fn compatible(ty: &<sqlx::Sqlite as sqlx::Database>::TypeInfo) -> bool {
        // Value is compatible with all SQLite types
        <i64 as sqlx::Type<sqlx::Sqlite>>::compatible(ty)
            || <f64 as sqlx::Type<sqlx::Sqlite>>::compatible(ty)
            || <String as sqlx::Type<sqlx::Sqlite>>::compatible(ty)
            || <Vec<u8> as sqlx::Type<sqlx::Sqlite>>::compatible(ty)
    }
}

#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Sqlite> for Value {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Sqlite as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        match self {
            Value::Null => Ok(sqlx::encode::IsNull::Yes),
            Value::Integer(i) => {
                buf.push(sqlx::sqlite::SqliteArgumentValue::Int64(*i));
                Ok(sqlx::encode::IsNull::No)
            }
            Value::Real(r) => {
                buf.push(sqlx::sqlite::SqliteArgumentValue::Double(*r));
                Ok(sqlx::encode::IsNull::No)
            }
            Value::Text(t) => {
                buf.push(sqlx::sqlite::SqliteArgumentValue::Text(Cow::Owned(
                    t.clone(),
                )));
                Ok(sqlx::encode::IsNull::No)
            }
            Value::Blob(b) => {
                buf.push(sqlx::sqlite::SqliteArgumentValue::Blob(Cow::Owned(
                    b.clone(),
                )));
                Ok(sqlx::encode::IsNull::No)
            }
        }
    }
}
