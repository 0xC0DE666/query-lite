#[cfg(feature = "sqlx")]
use query_lite::sql;
#[cfg(feature = "sqlx")]
use sqlx::Encode;

// ============================================================================
// SQLX Encode TESTS
// ============================================================================

#[cfg(feature = "sqlx")]
#[test]
fn test_value_encode_null() {
    let value = sql::Value::Null;
    let mut buf = Vec::new();
    let result = value.encode_by_ref(&mut buf);

    assert!(result.is_ok());
    match result.unwrap() {
        sqlx::encode::IsNull::Yes => {}
        sqlx::encode::IsNull::No => panic!("Expected IsNull::Yes"),
    }
    assert!(buf.is_empty());
}

#[cfg(feature = "sqlx")]
#[test]
fn test_value_encode_integer() {
    let value = sql::Value::Integer(42);
    let mut buf = Vec::new();
    let result = value.encode_by_ref(&mut buf);

    assert!(result.is_ok());
    match result.unwrap() {
        sqlx::encode::IsNull::No => {}
        sqlx::encode::IsNull::Yes => panic!("Expected IsNull::No"),
    }
    assert_eq!(buf.len(), 1);

    match &buf[0] {
        sqlx::sqlite::SqliteArgumentValue::Int64(i) => assert_eq!(*i, 42),
        _ => panic!("Expected Int64"),
    }
}

#[cfg(feature = "sqlx")]
#[test]
fn test_value_encode_integer_negative() {
    let value = sql::Value::Integer(-123);
    let mut buf = Vec::new();
    let result = value.encode_by_ref(&mut buf);

    assert!(result.is_ok());
    match result.unwrap() {
        sqlx::encode::IsNull::No => {}
        sqlx::encode::IsNull::Yes => panic!("Expected IsNull::No"),
    }

    match &buf[0] {
        sqlx::sqlite::SqliteArgumentValue::Int64(i) => assert_eq!(*i, -123),
        _ => panic!("Expected Int64"),
    }
}

#[cfg(feature = "sqlx")]
#[test]
fn test_value_encode_integer_large() {
    let value = sql::Value::Integer(9223372036854775807i64);
    let mut buf = Vec::new();
    let result = value.encode_by_ref(&mut buf);

    assert!(result.is_ok());
    match result.unwrap() {
        sqlx::encode::IsNull::No => {}
        sqlx::encode::IsNull::Yes => panic!("Expected IsNull::No"),
    }

    match &buf[0] {
        sqlx::sqlite::SqliteArgumentValue::Int64(i) => assert_eq!(*i, 9223372036854775807i64),
        _ => panic!("Expected Int64"),
    }
}

#[cfg(feature = "sqlx")]
#[test]
fn test_value_encode_real() {
    let value = sql::Value::Real(3.14);
    let mut buf = Vec::new();
    let result = value.encode_by_ref(&mut buf);

    assert!(result.is_ok());
    match result.unwrap() {
        sqlx::encode::IsNull::No => {}
        sqlx::encode::IsNull::Yes => panic!("Expected IsNull::No"),
    }

    match &buf[0] {
        sqlx::sqlite::SqliteArgumentValue::Double(d) => assert_eq!(*d, 3.14),
        _ => panic!("Expected Double"),
    }
}

#[cfg(feature = "sqlx")]
#[test]
fn test_value_encode_real_negative() {
    let value = sql::Value::Real(-123.456);
    let mut buf = Vec::new();
    let result = value.encode_by_ref(&mut buf);

    assert!(result.is_ok());
    match result.unwrap() {
        sqlx::encode::IsNull::No => {}
        sqlx::encode::IsNull::Yes => panic!("Expected IsNull::No"),
    }

    match &buf[0] {
        sqlx::sqlite::SqliteArgumentValue::Double(d) => assert_eq!(*d, -123.456),
        _ => panic!("Expected Double"),
    }
}

#[cfg(feature = "sqlx")]
#[test]
fn test_value_encode_text() {
    let value = sql::Value::Text("hello world".to_string());
    let mut buf = Vec::new();
    let result = value.encode_by_ref(&mut buf);

    assert!(result.is_ok());
    match result.unwrap() {
        sqlx::encode::IsNull::No => {}
        sqlx::encode::IsNull::Yes => panic!("Expected IsNull::No"),
    }

    match &buf[0] {
        sqlx::sqlite::SqliteArgumentValue::Text(cow) => {
            assert_eq!(cow.as_ref(), "hello world");
        }
        _ => panic!("Expected Text"),
    }
}

#[cfg(feature = "sqlx")]
#[test]
fn test_value_encode_text_empty() {
    let value = sql::Value::Text("".to_string());
    let mut buf = Vec::new();
    let result = value.encode_by_ref(&mut buf);

    assert!(result.is_ok());
    match result.unwrap() {
        sqlx::encode::IsNull::No => {}
        sqlx::encode::IsNull::Yes => panic!("Expected IsNull::No"),
    }

    match &buf[0] {
        sqlx::sqlite::SqliteArgumentValue::Text(cow) => {
            assert_eq!(cow.as_ref(), "");
        }
        _ => panic!("Expected Text"),
    }
}

#[cfg(feature = "sqlx")]
#[test]
fn test_value_encode_text_unicode() {
    let value = sql::Value::Text("测试".to_string());
    let mut buf = Vec::new();
    let result = value.encode_by_ref(&mut buf);

    assert!(result.is_ok());
    match result.unwrap() {
        sqlx::encode::IsNull::No => {}
        sqlx::encode::IsNull::Yes => panic!("Expected IsNull::No"),
    }

    match &buf[0] {
        sqlx::sqlite::SqliteArgumentValue::Text(cow) => {
            assert_eq!(cow.as_ref(), "测试");
        }
        _ => panic!("Expected Text"),
    }
}

#[cfg(feature = "sqlx")]
#[test]
fn test_value_encode_blob() {
    let value = sql::Value::Blob(vec![1, 2, 3, 4, 5]);
    let mut buf = Vec::new();
    let result = value.encode_by_ref(&mut buf);

    assert!(result.is_ok());
    match result.unwrap() {
        sqlx::encode::IsNull::No => {}
        sqlx::encode::IsNull::Yes => panic!("Expected IsNull::No"),
    }

    match &buf[0] {
        sqlx::sqlite::SqliteArgumentValue::Blob(cow) => {
            assert_eq!(cow.as_ref(), &[1, 2, 3, 4, 5]);
        }
        _ => panic!("Expected Blob"),
    }
}

#[cfg(feature = "sqlx")]
#[test]
fn test_value_encode_blob_empty() {
    let value = sql::Value::Blob(vec![]);
    let mut buf = Vec::new();
    let result = value.encode_by_ref(&mut buf);

    assert!(result.is_ok());
    match result.unwrap() {
        sqlx::encode::IsNull::No => {}
        sqlx::encode::IsNull::Yes => panic!("Expected IsNull::No"),
    }

    match &buf[0] {
        sqlx::sqlite::SqliteArgumentValue::Blob(cow) => {
            assert_eq!(cow.as_ref(), &[]);
        }
        _ => panic!("Expected Blob"),
    }
}
