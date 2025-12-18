# query-lite

[![Crates.io](https://img.shields.io/crates/v/query-lite.svg)](https://crates.io/crates/query-lite)
[![Documentation](https://docs.rs/query-lite/badge.svg)](https://docs.rs/query-lite)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/0xC0DE666/query-lite#license)

A convenient SQL query builder for rusqlite that makes it easy to build type-safe SQLite queries with parameter binding. Parse HTTP query parameters, build queries programmatically, or combine both approaches to create secure, parameterized SQL queries.

## Why query-lite?

Building SQL queries manually is error-prone and tedious. query-lite provides a type-safe, builder-pattern API that:

- ✅ **Prevents SQL Injection**: All queries use parameterized placeholders
- ✅ **Type-Safe**: Compile-time guarantees for query structure
- ✅ **Easy to Use**: Builder pattern with fluent API
- ✅ **rusqlite Ready**: Direct integration with rusqlite's `ToSql` trait
- ✅ **Flexible**: Build queries programmatically or parse from HTTP

### Example

```rust
use query_lite::{Query, Parameters, Order};
use rusqlite::Connection;

// Build a query
let mut params = Parameters::new();
params
    .contains("name".to_string(), vec!["john".to_string()])
    .between("age".to_string(), vec!["20".to_string(), "30".to_string()])
    .greater("price".to_string(), vec!["100".to_string()]);

let mut order = Order::new();
order.descending("date_created".to_string());

let query = Query::init(params, order, 25, 0);

// Generate SQL
let sql = query.to_sql();
// "WHERE name LIKE ? AND age BETWEEN ? AND ? AND price > ? ORDER BY date_created DESC LIMIT ? OFFSET ?"

// Get values for rusqlite
let values = query.to_values();

// Use with rusqlite
let conn = Connection::open("database.db")?;
let mut stmt = conn.prepare(&format!("SELECT * FROM users {}", sql))?;
let params: Vec<&dyn rusqlite::types::ToSql> = values
    .iter()
    .map(|v| v as &dyn rusqlite::types::ToSql)
    .collect();
let rows = stmt.query(params.as_slice())?;
```

## Features

- 🗄️ **SQL Query Builder**: Build type-safe SQLite queries with automatic parameter binding for rusqlite
- 🔒 **SQL Injection Safe**: All queries use parameterized placeholders - never string concatenation
- 🎯 **Rich Filtering**: Support for equals, contains, starts-with, ends-with, between, greater, lesser, and more
- 📊 **Sorting & Pagination**: Built-in support for ORDER BY, LIMIT, and OFFSET clauses
- 🔍 **HTTP Query Parsing**: Optional support for parsing HTTP query parameters into SQL queries
- 🛡️ **Type Safety**: Full Rust type safety with comprehensive error handling
- ⚡ **Zero-Copy Values**: Direct integration with rusqlite's `ToSql` trait for efficient parameter binding
- 🧪 **Well Tested**: Comprehensive test suite with 240+ tests

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
# SQL query builder (default - includes SQL generation)
query-lite = "0.11.0"

# With HTTP query parameter parsing (optional)
query-lite = { version = "0.11.0", features = ["http"] }
```

## Basic Usage

### Building SQL Queries Programmatically

The primary use case is building SQL queries for rusqlite:

```rust
use query_lite::{Query, Parameters, Order};
use rusqlite::Connection;

#[cfg(feature = "sql")]
fn example() -> Result<(), Box<dyn std::error::Error>> {
    // Build query programmatically
    let mut params = Parameters::new();
    params
        .contains("name".to_string(), vec!["john".to_string()])
        .between("age".to_string(), vec!["20".to_string(), "30".to_string()])
        .greater("price".to_string(), vec!["100".to_string()]);

    let mut order = Order::new();
    order.descending("date_created".to_string());

    let query = Query::init(params, order, 25, 0);

    // Generate SQL with parameter placeholders
    let sql = query.to_sql();
    // Result: "WHERE name LIKE ? AND age BETWEEN ? AND ? AND price > ? ORDER BY date_created DESC LIMIT ? OFFSET ?"

    // Get parameter values for rusqlite
    let values = query.to_values();
    // Result: [sql::Value::Text("%john%"), sql::Value::Integer(20), sql::Value::Integer(30), 
    //          sql::Value::Integer(100), sql::Value::Integer(25), sql::Value::Integer(0)]

    // Use with rusqlite
    let conn = Connection::open("database.db")?;
    let mut stmt = conn.prepare(&format!("SELECT * FROM users {}", sql))?;
    
    // sql::Value implements ToSql, so we can bind directly
    let params: Vec<&dyn rusqlite::types::ToSql> = values
        .iter()
        .map(|v| v as &dyn rusqlite::types::ToSql)
        .collect();
    
    let rows = stmt.query(params.as_slice())?;
    // Process rows...
    
    Ok(())
}
```

### Parsing HTTP Query Parameters (Optional)

If you enable the `http` feature, you can also parse HTTP query strings:

```rust
use query_lite::Query;
use rusqlite::Connection;

#[cfg(all(feature = "sql", feature = "http"))]
fn example() -> Result<(), Box<dyn std::error::Error>> {
    // Parse HTTP query parameters
    let query = Query::from_http("name=contains:john&age=between:20,30&order=date_created:desc&limit=25".to_string())?;

    // Generate SQL
    let sql = query.to_sql();
    let values = query.to_values();

    // Use with rusqlite
    let conn = Connection::open("database.db")?;
    let mut stmt = conn.prepare(&format!("SELECT * FROM users {}", sql))?;
    
    let params: Vec<&dyn rusqlite::types::ToSql> = values
        .iter()
        .map(|v| v as &dyn rusqlite::types::ToSql)
        .collect();
    
    let rows = stmt.query(params.as_slice())?;
    // Process rows...
    
    Ok(())
}
```

### Traditional Query Parameters

```rust
use query_lite::Query;

// Parse traditional HTTP query parameters
let query = Query::from_http("name=john&age=25&city=london".to_string())?;

// Access parameters
let name_param = query.parameters.inner().get("name").unwrap();
assert_eq!(*name_param.similarity(), query_lite::Similarity::Equals);
assert_eq!(name_param.values(), &vec!["john"]);

// Convert back to HTTP
let http_string = query.to_http();
// Result: "name=equals:john&age=equals:25&city=equals:london&limit=50&offset=0"
```

### Advanced Similarity-based Parameters

```rust
use query_lite::Query;

// Parse advanced query parameters
let query = Query::from_http("name=contains:john&age=between:20,30&price=greater:100".to_string())?;

// Access parameters with different similarity types
let name_param = query.parameters.inner().get("name").unwrap();
assert_eq!(*name_param.similarity(), query_lite::Similarity::Contains);
assert_eq!(name_param.values(), &vec!["john"]);

let age_param = query.parameters.inner().get("age").unwrap();
assert_eq!(*age_param.similarity(), query_lite::Similarity::Between);
assert_eq!(age_param.values(), &vec!["20", "30"]);
```

### Mixed Traditional and Advanced

```rust
use query_lite::Query;

// Mix traditional and advanced parameters
let query = Query::from_http("name=john&name=jane&age=contains:25&status=active".to_string())?;

// Traditional parameters (repeated values)
let name_param = query.parameters.inner().get("name").unwrap();
assert_eq!(*name_param.similarity(), query_lite::Similarity::Equals);
assert_eq!(name_param.values(), &vec!["john", "jane"]);

// Advanced parameters
let age_param = query.parameters.inner().get("age").unwrap();
assert_eq!(*age_param.similarity(), query_lite::Similarity::Contains);
assert_eq!(age_param.values(), &vec!["25"]);
```

## Programmatic Query Building

You can also build queries programmatically using the builder pattern:

```rust
use query_lite::{Query, Parameters, Order};

// Build parameters using the builder pattern
let mut parameters = Parameters::new();
parameters
    .equals("name".to_string(), vec!["john".to_string(), "jane".to_string()])
    .contains("description".to_string(), vec!["rust".to_string()])
    .between("age".to_string(), vec!["18".to_string(), "65".to_string()])
    .greater("price".to_string(), vec!["100".to_string()]);

// Build sort fields using the builder pattern
let mut order = Order::new();
order
    .descending("date_created".to_string())
    .ascending("name".to_string());

// Create the query
let query = Query::init(parameters, order, 25, 0);

// Convert to HTTP string
let http_string = query.to_http();
// Result: "name=equals:john,jane&description=contains:rust&age=between:18,65&price=greater:100&order=date_created:desc,name:asc&limit=25&offset=0"
```

## Enhanced Parameter Access

The library provides multiple ways to access parameter data for different use cases:

### Semantic Access (Recommended)

```rust
use query_lite::Query;

let query = Query::from_http("name=contains:john&age=between:20,30".to_string())?;

// Access parameters using semantic methods
let name_param = query.parameters.inner().get("name").unwrap();
assert_eq!(*name_param.similarity(), Similarity::Contains);
assert_eq!(name_param.values(), &vec!["john".to_string()]);

let age_param = query.parameters.inner().get("age").unwrap();
assert_eq!(*age_param.similarity(), Similarity::Between);
assert_eq!(age_param.values(), &vec!["20".to_string(), "30".to_string()]);
```

### Direct Collection Access

For advanced operations, you can access the underlying collections directly:

```rust
use query_lite::Query;

let mut query = Query::new();
query.parameters.equals("name".to_string(), vec!["john".to_string()]);
query.order.ascending("date_created".to_string());

// Access the underlying IndexMap for complex operations
let param_map = query.parameters.inner();
let order_map = query.order.inner();

// Iterate over all parameters
for (key, param) in param_map {
    println!("{}: {:?} = {:?}", key, param.similarity(), param.values());
}

// Perform bulk operations
let param_map_mut = query.parameters.inner_mut();
param_map_mut.insert("new_param".to_string(), Parameter::init(Similarity::Greater, vec!["100".to_string()]));
```

### Parameter Access

The library provides semantic access methods for parameter data:

```rust
use query_lite::Query;

let query = Query::from_http("name=contains:john".to_string())?;
let param = query.parameters.inner().get("name").unwrap();

// Use semantic access methods
assert_eq!(*param.similarity(), Similarity::Contains);
assert_eq!(param.values(), &vec!["john".to_string()]);

// Create parameters using the init method
let new_param = Parameter::init(Similarity::Greater, vec!["100".to_string()]);
```

## Similarity Types

The library supports various similarity types for advanced filtering:

| Similarity | Description | Example | SQLite Equivalent |
|------------|-------------|---------|----------------|
| `equals` | Exact match | `name=equals:john` | `name = ?` |
| `contains` | Substring match | `name=contains:john` | `name LIKE ?` |
| `starts-with` | Prefix match | `name=starts-with:john` | `name LIKE ?` |
| `ends-with` | Suffix match | `name=ends-with:john` | `name LIKE ?` |
| `between` | Range match | `age=between:20,30` | `age BETWEEN ? AND ?` |
| `greater` | Greater than | `price=greater:100` | `price > ?` |
| `lesser` | Less than | `price=lesser:100` | `price < ?` |
| `greater-or-equal` | Greater or equal | `price=greater-or-equal:100` | `price >= ?` |
| `lesser-or-equal` | Less or equal | `price=lesser-or-equal:100` | `price <= ?` |

### Multiple Values

```rust
// Multiple values for equals (IN clause)
"?name=equals:john,jane,bob"
// → name IN ('john', 'jane', 'bob')

// Multiple ranges for between
"?age=between:18,25,30,40,50,65"
// → (age BETWEEN 18 AND 25) OR (age BETWEEN 30 AND 40) OR (age BETWEEN 50 AND 65)
// Note: Odd values (65) are ignored
```

## Sorting and Pagination

```rust
use query_lite::Query;

let query = Query::from_http("name=john&order=date_created:desc,name:asc&limit=25&offset=10".to_string())?;

// Access sorting
assert_eq!(query.order.inner().len(), 2);
assert_eq!(query.order.inner().get("date_created"), Some(&query_lite::SortDirection::Descending));
assert_eq!(query.order.inner().get("name"), Some(&query_lite::SortDirection::Ascending));

// Access pagination
assert_eq!(query.limit, 25);
assert_eq!(query.offset, 10);
```

## SQL Query Building

The core feature of query-lite is building SQL queries for rusqlite. All queries use parameterized placeholders to prevent SQL injection:

```rust
use query_lite::Query;

let query = Query::from_http("name=contains:john&age=between:20,30&order=date_created:desc&limit=10".to_string())?;

// Generate SQLite-compatible SQL with parameter placeholders
let sql = query.to_sql();
// Result: "WHERE name LIKE ? AND age BETWEEN ? AND ? ORDER BY date_created DESC LIMIT ? OFFSET ?"

// Get parameter values separately for more control
let param_values = query.parameter_values();
let pagination_values = query.pagination_values();
let total_params = query.total_parameters();

// Use with SQLite
// let stmt = conn.prepare(&format!("SELECT * FROM users {}", sql))?;
// let rows = stmt.query(param_values)?;
```

### Advanced SQLite Clause Management

Version 0.8.0 introduces improved SQLite clause methods that return `Option<String>` for better semantic clarity:

```rust
use query_lite::Query;

let query = Query::from_http("name=contains:john&age=between:20,30&order=date_created:desc".to_string())?;

// Get WHERE clause (returns None if no conditions)
match query.where_clause() {
    Some(where_clause) => println!("WHERE {}", where_clause),
    None => println!("No WHERE conditions"),
}

// Get ORDER BY clause (returns None if no sorting)
match query.order_clause() {
    Some(order_clause) => println!("ORDER BY {}", order_clause),
    None => println!("No ORDER BY clause"),
}

// Build custom SQL with explicit handling
let sql = match (query.where_clause(), query.order_clause()) {
    (Some(where_clause), Some(order_clause)) => 
        format!("SELECT * FROM users WHERE {} ORDER BY {}", where_clause, order_clause),
    (Some(where_clause), None) => 
        format!("SELECT * FROM users WHERE {}", where_clause),
    (None, Some(order_clause)) => 
        format!("SELECT * FROM users ORDER BY {}", order_clause),
    (None, None) => 
        "SELECT * FROM users".to_string(),
};
```

### Advanced SQLite Value Management

Version 0.6.0 introduces simplified SQLite value methods:

```rust
use query_lite::Query;

let query = Query::from_http("name=contains:john&age=between:20,30&price=greater:100&order=date_created:desc".to_string())?;

// Get only parameter values (without pagination)
let param_values = query.parameter_values();
// Result: [sql::Value::Text("%john%"), sql::Value::Text("20"), sql::Value::Text("30"), sql::Value::Text("100")]

// Get only order values (without pagination)
let param_values = query.parameter_values();
// Result: [sql::Value::Text("%john%"), sql::Value::Text("20"), sql::Value::Text("30"), sql::Value::Text("100")]

// Get only pagination values
let pagination_values = query.pagination_values();
// Result: [sql::Value::Integer(50), sql::Value::Integer(0)]

// Get total parameter count
let total_params = query.total_parameters();
// Result: 6 (4 parameter values + 2 pagination values)

// Combine for complete SQL execution
let all_values = [param_values, pagination_values].concat();
// Use with SQLite
// let stmt = conn.prepare(&format!("SELECT * FROM users {}", query.to_sql()))?;
// let rows = stmt.query(all_values)?;
```

This granular approach allows for:
- **Separate Parameter Handling**: Process parameter values and pagination values independently
- **Custom Value Processing**: Apply different logic to parameters vs pagination
- **Performance Optimization**: Avoid unnecessary value processing when only certain parts are needed
- **Debugging**: Easily inspect parameter counts and values for troubleshooting

### SQLite Examples

The generated SQL uses SQLite syntax with `?` parameter placeholders:

```rust
// Traditional parameters
"?name=john&name=jane&age=25"
// → "WHERE name IN (?, ?) AND age = ? LIMIT ? OFFSET ?"

// Advanced parameters
"?name=contains:john&age=between:20,30&price=greater:100"
// → "WHERE name LIKE ? AND age BETWEEN ? AND ? AND price > ? LIMIT ? OFFSET ?"

// Complex mixed query
"?name=john&name=jane&age=contains:25&price=greater:100&order=date_created:desc&limit=20"
// → "WHERE name IN (?, ?) AND age LIKE ? AND price > ? ORDER BY date_created DESC LIMIT ? OFFSET ?"
```

### rusqlite Integration

The `sql::Value` type (which is `rusqlite::types::Value`) implements `rusqlite::types::ToSql`, allowing direct parameter binding to rusqlite queries:

```rust
use query_lite::Query;
use rusqlite::Connection;

#[cfg(feature = "sql")]
fn example() -> Result<(), Box<dyn std::error::Error>> {
    let query = Query::from_http("name=contains:john&age=between:20,30".to_string())?;
    let sql = format!("SELECT * FROM users {}", query.to_sql());
    
    let conn = Connection::open_in_memory()?;
    
    // sql::Value implements ToSql, so you can bind directly
    let params: Vec<&dyn rusqlite::types::ToSql> = query.parameter_values()
        .iter()
        .map(|v| v as &dyn rusqlite::types::ToSql)
        .collect();
    
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query(params.as_slice())?;
    // Process rows...
    Ok(())
}
```

## URL Encoding Support

The library automatically handles URL encoding and decoding:

```rust
use query_lite::Query;

// URL encoded parameters
let query = Query::from_http("name=john%20doe&email=test%40example.com".to_string())?;

let name_param = query.parameters.inner().get("name").unwrap();
assert_eq!(name_param.values(), &vec!["john doe"]); // Automatically decoded

let email_param = query.parameters.inner().get("email").unwrap();
assert_eq!(email_param.values(), &vec!["test@example.com"]); // Automatically decoded
```

## Query Manipulation

```rust
use query_lite::Query;

let query = Query::from_http("name=john&age=25&email=john@example.com".to_string())?;

// Keep only specific parameters
let filtered_params = query.parameters.keep(vec!["name".to_string(), "age".to_string()]);
let filtered_query = Query::init(filtered_params, query.order, query.limit, query.offset);
// Result: Only name and age parameters remain

// Remove specific parameters
let filtered_params = query.parameters.remove(vec!["email".to_string()]);
let filtered_query = Query::init(filtered_params, query.order, query.limit, query.offset);
// Result: email parameter is removed, name and age remain
```

## Error Handling

```rust
use query_lite::{Query, error::Error};

match Query::from_http("invalid=query".to_string()) {
    Ok(query) => {
        // Handle successful parsing
        println!("Query parsed successfully: {:?}", query);
    }
    Err(Error::InvalidParameter(msg)) => {
        // Handle invalid parameter format
        eprintln!("Invalid parameter: {}", msg);
    }
    Err(Error::InvalidOrderField(msg)) => {
        // Handle invalid order field
        eprintln!("Invalid order field: {}", msg);
    }
    Err(e) => {
        // Handle other errors
        eprintln!("Error: {}", e);
    }
}
```

## Real-world Examples

### E-commerce Product Search

```rust
use query_lite::Query;

// Complex product search with multiple filters
let query = Query::from_http(
    "category=electronics&brand=apple&brand=samsung&price=between:100,500&rating=greater-or-equal:4&order=price:asc&limit=20"
)?;

// Generate SQLite query for product search
let sql = query.to_sql();
// "WHERE category = ? AND brand IN (?, ?) AND price BETWEEN ? AND ? AND rating >= ? ORDER BY price ASC LIMIT ? OFFSET ?"
```

### User Management System

```rust
use query_lite::Query;

// User filtering and management
let query = Query::from_http(
    "name=contains:john&age=greater:18&status=active&role=admin&role=user&order=created_at:desc&limit=50"
)?;

// Generate SQLite query for user query
let sql = query.to_sql();
// "WHERE name LIKE ? AND age > ? AND status = ? AND role IN (?, ?) ORDER BY created_at DESC LIMIT ? OFFSET ?"
```

### Content Management

```rust
use query_lite::Query;

// Content filtering with date ranges
let query = Query::from_http(
    "title=contains:rust&tags=programming&tags=web&date=between:2023-01-01,2023-12-31&published=true&order=date:desc&limit=25"
)?;

// Generate SQLite query for content query
let sql = query.to_sql();
// "WHERE title LIKE ? AND tags IN (?, ?) AND date BETWEEN ? AND ? AND published = ? ORDER BY date DESC LIMIT ? OFFSET ?"
```

## Feature Flags

The library supports feature flags for optional functionality:

```toml
[dependencies]
# Default: SQL query builder (includes SQL generation)
query-lite = "0.11.0"

# With HTTP query parameter parsing (optional)
query-lite = { version = "0.11.0", features = ["http"] }
```

### Feature Details

- **`sql`** (default): Enables SQL query generation methods (`to_sql()`, `where_clause()`, `order_clause()`, etc.) and re-exports `rusqlite::types::Value` as `sql::Value`. The `sql::Value` type implements `rusqlite::types::ToSql`, allowing direct parameter binding to rusqlite queries.
- **`http`** (optional): Enables HTTP query string parsing and generation methods (`from_http()`, `to_http()`).

## API Reference

### Core Types

- `Query`: Main query structure containing parameters, sorting, and pagination
- `Parameters`: Collection of query parameters with builder methods
- `Parameter`: Struct containing similarity and values with semantic access methods (fields are private)
- `Order`: Collection of sort fields with builder methods
- `Similarity`: Enum defining comparison types (equals, contains, between, etc.)
- `SortDirection`: Sort direction (ascending, descending)

### Key Methods

#### Query Methods
- `Query::new()`: Create a new Query with default values (empty parameters, empty order, limit=50, offset=0)
- `Query::init()`: Create Query with custom parameters, order, limit, and offset
- `Query::to_sql()`: Generate SQLite-compatible query with parameter placeholders (default feature)
- `Query::from_http()`: Parse HTTP query string into Query struct (requires `http` feature)
- `Query::to_http()`: Convert Query struct back to HTTP query string (requires `http` feature)
- `Query::where_clause()`: Get WHERE clause as Option<String> (feature-gated)
- `Query::order_clause()`: Get ORDER BY clause as Option<String> (feature-gated)
- `Query::to_values()`: Get all SQLite values (parameters + pagination) (feature-gated)
- `Query::parameter_values()`: Get SQLite values for parameters only (feature-gated)
- `Query::pagination_values()`: Get SQLite values for pagination only (feature-gated)
- `Query::total_parameters()`: Get total number of SQLite parameter values (feature-gated)

#### Parameters Methods
- `Parameters::new()`: Create new Parameters collection
- `Parameters::equals()`, `Parameters::contains()`, etc.: Builder methods for adding parameters
- `Parameters::inner()`: Get immutable reference to underlying IndexMap
- `Parameters::inner_mut()`: Get mutable reference to underlying IndexMap
- `Parameters::keep()`: Filter parameters to keep only specified keys
- `Parameters::remove()`: Remove specified parameters

#### Parameter Methods
- `Parameter::init()`: Create a new Parameter with similarity and values
- `Parameter::similarity()`: Get reference to similarity type
- `Parameter::values()`: Get reference to parameter values
- `Parameter::values_mut()`: Get mutable reference to parameter values

#### Order Methods
- `Order::new()`: Create new Order collection
- `Order::ascending()`, `Order::descending()`: Builder methods for adding sort fields
- `Order::inner()`: Get immutable reference to underlying IndexMap
- `Order::inner_mut()`: Get mutable reference to underlying IndexMap
- `Order::keep()`: Filter sort fields to keep only specified keys
- `Order::remove()`: Remove specified sort fields

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a detailed list of changes.

---

**Made with ❤️ in Rust**
