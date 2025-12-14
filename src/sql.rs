use std::borrow::Cow;

pub const NULL: &str = "null";

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
impl<'q> sqlx::Encode<'q, sqlx::Sqlite> for Value {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Sqlite as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        match self {
            Value::Null => {}
            Value::Integer(i) => buf.push(sqlx::sqlite::SqliteArgumentValue::Int64(*i)),
            Value::Real(r) => buf.push(sqlx::sqlite::SqliteArgumentValue::Double(*r)),
            Value::Text(t) => buf.push(sqlx::sqlite::SqliteArgumentValue::Text(Cow::Owned(
                t.clone(),
            ))),
            Value::Blob(b) => buf.push(sqlx::sqlite::SqliteArgumentValue::Blob(Cow::Owned(
                b.clone(),
            ))),
        };
        if *self == Value::Null {
            Ok(sqlx::encode::IsNull::Yes)
        } else {
            Ok(sqlx::encode::IsNull::No)
        }
    }
}
