#[cfg(feature = "sqlx")]
use query_lite::sql;
#[cfg(feature = "sqlx")]
use sqlx::{Encode, Row};

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
    assert_eq!(buf.len(), 0);
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

// ============================================================================
// SQLX Type Trait Tests (verifies Type trait implementation works with bind)
// ============================================================================

#[cfg(feature = "sqlx")]
#[tokio::test]
async fn test_value_type_trait_with_bind() {
    use sqlx::SqlitePool;

    let pool = SqlitePool::connect(":memory:").await.unwrap();

    sqlx::query("CREATE TABLE test (id INTEGER PRIMARY KEY, value INTEGER)")
        .execute(&pool)
        .await
        .unwrap();

    // Test that we can bind sql::Value directly (requires Type trait)
    let value = sql::Value::Integer(42);
    sqlx::query("INSERT INTO test (value) VALUES (?)")
        .bind(&value)
        .execute(&pool)
        .await
        .unwrap();

    let result: i64 = sqlx::query_scalar("SELECT value FROM test WHERE id = 1")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(result, 42);
}

#[cfg(feature = "sqlx")]
#[tokio::test]
async fn test_value_type_trait_all_types() {
    use sqlx::SqlitePool;

    let pool = SqlitePool::connect(":memory:").await.unwrap();

    sqlx::query(
        "CREATE TABLE test (
            id INTEGER PRIMARY KEY,
            int_val INTEGER,
            real_val REAL,
            text_val TEXT,
            blob_val BLOB
        )",
    )
    .execute(&pool)
    .await
    .unwrap();

    // Test binding all Value types
    let values = vec![
        sql::Value::Integer(42),
        sql::Value::Real(3.14),
        sql::Value::Text("hello".to_string()),
        sql::Value::Blob(vec![1, 2, 3]),
    ];

    sqlx::query("INSERT INTO test (int_val, real_val, text_val, blob_val) VALUES (?, ?, ?, ?)")
        .bind(&values[0])
        .bind(&values[1])
        .bind(&values[2])
        .bind(&values[3])
        .execute(&pool)
        .await
        .unwrap();

    let row = sqlx::query("SELECT int_val, real_val, text_val, blob_val FROM test WHERE id = 1")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(row.get::<i64, _>(0), 42);
    assert_eq!(row.get::<f64, _>(1), 3.14);
    assert_eq!(row.get::<String, _>(2), "hello");
    assert_eq!(row.get::<Vec<u8>, _>(3), vec![1, 2, 3]);
}

#[cfg(feature = "sqlx")]
#[tokio::test]
async fn test_value_type_trait_with_query_lite() {
    use query_lite::Query;
    use sqlx::SqlitePool;

    let pool = SqlitePool::connect(":memory:").await.unwrap();

    sqlx::query("CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT, age INTEGER)")
        .execute(&pool)
        .await
        .unwrap();

    // Insert some test data
    sqlx::query("INSERT INTO test (name, age) VALUES ('john', 25), ('jane', 35)")
        .execute(&pool)
        .await
        .unwrap();

    // Test with actual Query from query-lite
    let query = Query::from_http("name=contains:john&age=between:20,30".to_string()).unwrap();

    // Build the SQL query properly
    let mut sql = "SELECT * FROM test".to_string();
    if let Some(where_clause) = query.where_clause() {
        sql.push_str(" WHERE ");
        sql.push_str(&where_clause);
    }
    if let Some(order_clause) = query.order_clause() {
        sql.push_str(" ORDER BY ");
        sql.push_str(&order_clause);
    }
    sql.push_str(" LIMIT ? OFFSET ?");

    // Build query and bind parameters - this requires Type trait
    let mut sqlx_query = sqlx::query(&sql);
    for param in query.parameter_values() {
        sqlx_query = sqlx_query.bind(param);
    }
    // Also bind pagination
    for param in query.pagination_values() {
        sqlx_query = sqlx_query.bind(param);
    }

    // This should compile and execute without errors
    let rows = sqlx_query.fetch_all(&pool).await.unwrap();
    assert_eq!(rows.len(), 1); // Should find john with age 25
}
