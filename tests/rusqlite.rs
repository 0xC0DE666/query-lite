#[cfg(feature = "rusqlite")]
use query_lite::sql;
#[cfg(feature = "rusqlite")]
use rusqlite::types::ToSql;

// ============================================================================
// RUSQLITE ToSql TESTS
// ============================================================================

#[cfg(feature = "rusqlite")]
#[test]
fn test_value_tosql_null() {
    let value = sql::Value::Null;
    let to_sql_output = value.to_sql().unwrap();

    match to_sql_output {
        rusqlite::types::ToSqlOutput::Borrowed(rusqlite::types::ValueRef::Null) => {}
        _ => panic!("Expected Null ValueRef"),
    }
}

#[cfg(feature = "rusqlite")]
#[test]
fn test_value_tosql_integer() {
    let value = sql::Value::Integer(42);
    let to_sql_output = value.to_sql().unwrap();

    match to_sql_output {
        rusqlite::types::ToSqlOutput::Borrowed(rusqlite::types::ValueRef::Integer(i)) => {
            assert_eq!(i, 42);
        }
        _ => panic!("Expected Integer ValueRef"),
    }
}

#[cfg(feature = "rusqlite")]
#[test]
fn test_value_tosql_integer_negative() {
    let value = sql::Value::Integer(-123);
    let to_sql_output = value.to_sql().unwrap();

    match to_sql_output {
        rusqlite::types::ToSqlOutput::Borrowed(rusqlite::types::ValueRef::Integer(i)) => {
            assert_eq!(i, -123);
        }
        _ => panic!("Expected Integer ValueRef"),
    }
}

#[cfg(feature = "rusqlite")]
#[test]
fn test_value_tosql_integer_large() {
    let value = sql::Value::Integer(9223372036854775807i64);
    let to_sql_output = value.to_sql().unwrap();

    match to_sql_output {
        rusqlite::types::ToSqlOutput::Borrowed(rusqlite::types::ValueRef::Integer(i)) => {
            assert_eq!(i, 9223372036854775807i64);
        }
        _ => panic!("Expected Integer ValueRef"),
    }
}

#[cfg(feature = "rusqlite")]
#[test]
fn test_value_tosql_real() {
    let value = sql::Value::Real(3.14);
    let to_sql_output = value.to_sql().unwrap();

    match to_sql_output {
        rusqlite::types::ToSqlOutput::Borrowed(rusqlite::types::ValueRef::Real(r)) => {
            assert_eq!(r, 3.14);
        }
        _ => panic!("Expected Real ValueRef"),
    }
}

#[cfg(feature = "rusqlite")]
#[test]
fn test_value_tosql_real_negative() {
    let value = sql::Value::Real(-123.456);
    let to_sql_output = value.to_sql().unwrap();

    match to_sql_output {
        rusqlite::types::ToSqlOutput::Borrowed(rusqlite::types::ValueRef::Real(r)) => {
            assert_eq!(r, -123.456);
        }
        _ => panic!("Expected Real ValueRef"),
    }
}

#[cfg(feature = "rusqlite")]
#[test]
fn test_value_tosql_text() {
    let value = sql::Value::Text("hello world".to_string());
    let to_sql_output = value.to_sql().unwrap();

    match to_sql_output {
        rusqlite::types::ToSqlOutput::Borrowed(rusqlite::types::ValueRef::Text(bytes)) => {
            assert_eq!(bytes, b"hello world");
        }
        _ => panic!("Expected Text ValueRef"),
    }
}

#[cfg(feature = "rusqlite")]
#[test]
fn test_value_tosql_text_empty() {
    let value = sql::Value::Text("".to_string());
    let to_sql_output = value.to_sql().unwrap();

    match to_sql_output {
        rusqlite::types::ToSqlOutput::Borrowed(rusqlite::types::ValueRef::Text(bytes)) => {
            assert_eq!(bytes, b"");
        }
        _ => panic!("Expected Text ValueRef"),
    }
}

#[cfg(feature = "rusqlite")]
#[test]
fn test_value_tosql_text_unicode() {
    let value = sql::Value::Text("测试".to_string());
    let to_sql_output = value.to_sql().unwrap();

    match to_sql_output {
        rusqlite::types::ToSqlOutput::Borrowed(rusqlite::types::ValueRef::Text(bytes)) => {
            assert_eq!(std::str::from_utf8(bytes).unwrap(), "测试");
        }
        _ => panic!("Expected Text ValueRef"),
    }
}

#[cfg(feature = "rusqlite")]
#[test]
fn test_value_tosql_blob() {
    let value = sql::Value::Blob(vec![1, 2, 3, 4, 5]);
    let to_sql_output = value.to_sql().unwrap();

    match to_sql_output {
        rusqlite::types::ToSqlOutput::Borrowed(rusqlite::types::ValueRef::Blob(bytes)) => {
            assert_eq!(bytes, &[1, 2, 3, 4, 5]);
        }
        _ => panic!("Expected Blob ValueRef"),
    }
}

#[cfg(feature = "rusqlite")]
#[test]
fn test_value_tosql_blob_empty() {
    let value = sql::Value::Blob(vec![]);
    let to_sql_output = value.to_sql().unwrap();

    match to_sql_output {
        rusqlite::types::ToSqlOutput::Borrowed(rusqlite::types::ValueRef::Blob(bytes)) => {
            assert_eq!(bytes, &[]);
        }
        _ => panic!("Expected Blob ValueRef"),
    }
}
