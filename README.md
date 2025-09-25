# query-x

[![Crates.io](https://img.shields.io/crates/v/query-x.svg)](https://crates.io/crates/query-x)
[![Documentation](https://docs.rs/query-x/badge.svg)](https://docs.rs/query-x)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/0xC0DE666/query-x#license)

A powerful Rust library for parsing HTTP query parameters into structured queries with support for both traditional and advanced similarity-based filtering, plus optional SQL generation.

## Features

- 🔍 **Dual URL Support**: Handle both traditional (`?name=john`) and advanced (`?name=contains:john`) query parameters
- 🎯 **Advanced Filtering**: Support for contains, starts-with, ends-with, between, greater, lesser, and more
- 🔄 **Roundtrip Conversion**: Convert between HTTP queries and structured objects seamlessly
- 🗄️ **SQL Generation**: Optional SQL query generation with parameter binding (feature-gated)
- 🛡️ **Type Safety**: Full Rust type safety with comprehensive error handling
- ⚡ **Zero Dependencies**: Minimal dependencies for core functionality
- 🧪 **Well Tested**: Comprehensive test suite with 150+ tests

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
query-x = "0.2.1"

# Optional: Enable SQL generation (enabled by default)
# query-x = { version = "0.2.1", default-features = false }
```

## Basic Usage

### Traditional Query Parameters

```rust
use query_x::Query;

// Parse traditional HTTP query parameters
let query = Query::from_http("name=john&age=25&city=london".to_string())?;

// Access parameters
let name_param = query.parameters.0.get("name").unwrap();
assert_eq!(name_param.0, query_x::Similarity::Equals);
assert_eq!(name_param.1, vec!["john"]);

// Convert back to HTTP
let http_string = query.to_http();
// Result: "name=equals:john&age=equals:25&city=equals:london&limit=50&offset=0"
```

### Advanced Similarity-based Parameters

```rust
use query_x::Query;

// Parse advanced query parameters
let query = Query::from_http("name=contains:john&age=between:20,30&price=greater:100".to_string())?;

// Access parameters with different similarity types
let name_param = query.parameters.0.get("name").unwrap();
assert_eq!(name_param.0, query_x::Similarity::Contains);
assert_eq!(name_param.1, vec!["john"]);

let age_param = query.parameters.0.get("age").unwrap();
assert_eq!(age_param.0, query_x::Similarity::Between);
assert_eq!(age_param.1, vec!["20", "30"]);
```

### Mixed Traditional and Advanced

```rust
use query_x::Query;

// Mix traditional and advanced parameters
let query = Query::from_http("name=john&name=jane&age=contains:25&status=active".to_string())?;

// Traditional parameters (repeated values)
let name_param = query.parameters.0.get("name").unwrap();
assert_eq!(name_param.0, query_x::Similarity::Equals);
assert_eq!(name_param.1, vec!["john", "jane"]);

// Advanced parameters
let age_param = query.parameters.0.get("age").unwrap();
assert_eq!(age_param.0, query_x::Similarity::Contains);
assert_eq!(age_param.1, vec!["25"]);
```

## Programmatic Query Building

You can also build queries programmatically using the builder pattern:

```rust
use query_x::{Query, Parameters, SortFields};

// Build parameters using the builder pattern
let mut parameters = Parameters::new();
parameters
    .equals("name".to_string(), vec!["john".to_string(), "jane".to_string()])
    .contains("description".to_string(), vec!["rust".to_string()])
    .between("age".to_string(), vec!["18".to_string(), "65".to_string()])
    .greater("price".to_string(), vec!["100".to_string()]);

// Build sort fields using the builder pattern
let mut sort_fields = SortFields::new();
sort_fields
    .descending("date_created".to_string())
    .ascending("name".to_string());

// Create the query
let query = Query::init(parameters, sort_fields, 25, 0);

// Convert to HTTP string
let http_string = query.to_http();
// Result: "name=equals:john,jane&description=contains:rust&age=between:18,65&price=greater:100&order=date_created:desc,name:asc&limit=25&offset=0"
```

## Similarity Types

The library supports various similarity types for advanced filtering:

| Similarity | Description | Example | SQL Equivalent |
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
use query_x::Query;

let query = Query::from_http("name=john&order=date_created:desc,name:asc&limit=25&offset=10".to_string())?;

// Access sorting
assert_eq!(query.sort_fields.0.len(), 2);
assert_eq!(query.sort_fields.0[0].name, "date_created");
assert_eq!(query.sort_fields.0[0].order, query_x::SortOrder::Descending);

// Access pagination
assert_eq!(query.limit, 25);
assert_eq!(query.offset, 10);
```

## SQL Generation (Optional)

Enable the `sql` feature (enabled by default) to generate SQL queries:

```rust
use query_x::Query;

let query = Query::from_http("name=contains:john&age=between:20,30&order=date_created:desc&limit=10".to_string())?;

// Generate SQL with parameter placeholders
let sql = query.to_sql();
// Result: "WHERE name LIKE ? AND age BETWEEN ? AND ? ORDER BY date_created DESC LIMIT ? OFFSET ?"

// Use with your database driver
// let stmt = conn.prepare(&format!("SELECT * FROM users {}", sql))?;
// let rows = stmt.query(["%john%", "20", "30", "10", "0"])?;
```

### SQL Examples

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

## URL Encoding Support

The library automatically handles URL encoding and decoding:

```rust
use query_x::Query;

// URL encoded parameters
let query = Query::from_http("name=john%20doe&email=test%40example.com".to_string())?;

let name_param = query.parameters.0.get("name").unwrap();
assert_eq!(name_param.1, vec!["john doe"]); // Automatically decoded

let email_param = query.parameters.0.get("email").unwrap();
assert_eq!(email_param.1, vec!["test@example.com"]); // Automatically decoded
```

## Query Manipulation

```rust
use query_x::Query;

let query = Query::from_http("name=john&age=25&email=john@example.com".to_string())?;

// Keep only specific parameters
let filtered_params = query.parameters.keep(vec!["name".to_string(), "age".to_string()]);
let filtered_query = Query::init(filtered_params, query.sort_fields, query.limit, query.offset);
// Result: Only name and age parameters remain

// Remove specific parameters
let filtered_params = query.parameters.remove(vec!["email".to_string()]);
let filtered_query = Query::init(filtered_params, query.sort_fields, query.limit, query.offset);
// Result: email parameter is removed, name and age remain
```

## Error Handling

```rust
use query_x::{Query, error::Error};

match Query::from_http("invalid=query".to_string()) {
    Ok(query) => {
        // Handle successful parsing
        println!("Query parsed successfully: {:?}", query);
    }
    Err(Error::InvalidParameter(msg)) => {
        // Handle invalid parameter format
        eprintln!("Invalid parameter: {}", msg);
    }
    Err(Error::InvalidSortField(msg)) => {
        // Handle invalid sort field
        eprintln!("Invalid sort field: {}", msg);
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
use query_x::Query;

// Complex product search with multiple filters
let query = Query::from_http(
    "category=electronics&brand=apple&brand=samsung&price=between:100,500&rating=greater-or-equal:4&order=price:asc&limit=20"
)?;

// Generate SQL for product search
let sql = query.to_sql();
// "WHERE category = ? AND brand IN (?, ?) AND price BETWEEN ? AND ? AND rating >= ? ORDER BY price ASC LIMIT ? OFFSET ?"
```

### User Management System

```rust
use query_x::Query;

// User filtering and management
let query = Query::from_http(
    "name=contains:john&age=greater:18&status=active&role=admin&role=user&order=created_at:desc&limit=50"
)?;

// Generate SQL for user query
let sql = query.to_sql();
// "WHERE name LIKE ? AND age > ? AND status = ? AND role IN (?, ?) ORDER BY created_at DESC LIMIT ? OFFSET ?"
```

### Content Management

```rust
use query_x::Query;

// Content filtering with date ranges
let query = Query::from_http(
    "title=contains:rust&tags=programming&tags=web&date=between:2023-01-01,2023-12-31&published=true&order=date:desc&limit=25"
)?;

// Generate SQL for content query
let sql = query.to_sql();
// "WHERE title LIKE ? AND tags IN (?, ?) AND date BETWEEN ? AND ? AND published = ? ORDER BY date DESC LIMIT ? OFFSET ?"
```

## Feature Flags

The library supports feature flags for optional functionality:

```toml
[dependencies]
# Default: includes SQL generation
query-x = "0.2.1"

# Without SQL generation (smaller binary)
query-x = { version = "0.2.1", default-features = false }

# With specific features
query-x = { version = "0.2.1", features = ["sql"] }
```

## API Reference

### Core Types

- `Query`: Main query structure containing parameters, sorting, and pagination
- `Parameters`: Collection of query parameters with builder methods
- `SortFields`: Collection of sort fields with builder methods
- `Similarity`: Enum defining comparison types (equals, contains, between, etc.)
- `SortOrder`: Sort direction (ascending, descending)

### Key Methods

- `Query::from_http()`: Parse HTTP query string into Query struct
- `Query::to_http()`: Convert Query struct back to HTTP query string
- `Query::to_sql()`: Generate SQL query with parameter placeholders (feature-gated)
- `Query::init()`: Create Query with custom parameters, sort fields, limit, and offset
- `Parameters::new()`: Create new Parameters collection
- `Parameters::equals()`, `Parameters::contains()`, etc.: Builder methods for adding parameters
- `SortFields::new()`: Create new SortFields collection
- `SortFields::ascending()`, `SortFields::descending()`: Builder methods for adding sort fields
- `Parameters::keep()`: Filter parameters to keep only specified keys
- `Parameters::remove()`: Remove specified parameters
- `SortFields::keep()`: Filter sort fields to keep only specified keys
- `SortFields::remove()`: Remove specified sort fields

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
