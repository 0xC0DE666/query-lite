# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.12.0] - 2026-02-05
- **Upgrades**: Upgraded rusqlite to v0.38.0.

## [0.12.0] - 2025-12-14

### Changed
- **Documentation Reframing**: Updated documentation to position the library as a SQL query builder for rusqlite
  - Reframed primary use case from HTTP query parameter parsing to SQL query building
  - Updated Cargo.toml description to emphasize SQL query building for rusqlite
  - Reorganized README.md to showcase SQL query building as the primary feature
  - HTTP query parameter parsing is now presented as an optional convenience feature
  - Updated keywords and categories to better reflect the library's purpose
  - Added prominent "Why query-lite?" section with SQL query building example
  - Reordered features list to emphasize SQL query building first
  - Updated Quick Start to show SQL query builder as default, HTTP as optional

### Technical Details
- **Better Positioning**: Library is now clearly positioned as a SQL query builder tool
- **Improved Discoverability**: Updated keywords help users find the library when searching for SQL query builders
- **Clearer Value Proposition**: Documentation now clearly communicates the primary use case
- **No API Changes**: All functionality remains the same, only documentation and positioning changed
- **Backward Compatible**: All existing code continues to work without any changes

## [0.11.0] - 2025-12-14

### Changed
- **SQL Value Trait Implementations**: Simplified SQL value trait implementations to encoding-only
  - Removed `rusqlite::types::FromSql` implementation for `sql::Value`
  - Removed `sqlx::Decode` implementation for `sql::Value`
  - Removed `sqlx::Type` implementation for `sql::Value` (only needed for Decode)
  - Kept `rusqlite::types::ToSql` implementation for encoding to rusqlite
  - Kept `sqlx::Encode` implementation for encoding to sqlx
  - `sql::Value` is now focused solely on encoding query parameters into SQL, not decoding results
  - Results from database queries should be decoded into domain types, not back into `sql::Value`

### Removed
- **Decode Functionality**: Removed ability to decode database values back into `sql::Value` enum
  - `rusqlite::types::FromSql` trait implementation removed
  - `sqlx::Decode` trait implementation removed
  - `sqlx::Type` trait implementation removed
  - All decode-related tests removed from `tests/rusqlite.rs` and `tests/sqlx.rs`
  - Encoding tests remain comprehensive and fully functional

### Technical Details
- **Simplified API**: `sql::Value` now has a single, clear purpose: encoding query parameters
- **Better Performance**: Removed unnecessary decode logic and edge case handling
- **Clearer Intent**: The enum's purpose is now unambiguous - it's for encoding, not storage
- **Reduced Complexity**: Eliminated complex type detection and multi-type decoding logic
- **Maintained Functionality**: All encoding functionality remains intact and fully tested
- **Version Bump**: Minor version bump reflects API simplification and removal of decode functionality

## [0.10.0] - 2025-11-30

### Changed
- **Type Renaming**: Renamed `SortOrder` enum to `SortDirection` for better clarity
  - `SortOrder` → `SortDirection` (enum name)
  - `OrderField::order()` → `OrderField::sort_direction()` (method name)
  - `Error::InvalidSortOrder` → `Error::InvalidSortDirection` (error variant)
  - Better semantic clarity: "Direction" more accurately describes ascending/descending than "Order"
  - Reduces confusion with the `Order` type (collection of sort fields)
- **Display Trait Implementation**: Replaced `ToString` implementations with `Display` trait for better Rust idioms
  - `Parameter`, `OrderField`, `Similarity`, `SortDirection` now implement `Display` instead of `ToString`
  - `Parameters` and `Order` now implement `Display` for HTTP query string formatting
  - `Display` automatically provides `ToString` via blanket implementation, maintaining backward compatibility
  - Enables direct use in `format!` macros: `format!("{}", parameter)` instead of `format!("{}", parameter.to_string())`
- **HTTP Generation Refactoring**: Simplified `Query::to_http()` method
  - Now uses `Display` implementations for `Parameters` and `Order` instead of inline formatting
  - Cleaner, more maintainable code with better separation of concerns
  - Formatting logic moved into type-specific `Display` implementations
- **SQL Module Organization**: Refactored SQL types into a dedicated module
  - `SqlValue` enum moved to `sql::Value` inside a new `pub mod sql` module
  - Extracted `"null"` string literal into `sql::NULL` constant for better maintainability
  - All SQL-related types now grouped under the `sql` module namespace
  - Improved code organization and module structure
  - Migration: Update `SqlValue::` to `sql::Value::` and `"null"` to `sql::NULL`
- **Test Organization**: Moved internal parsing tests to integration tests
  - Tests for `Parameter::from_str()` and `OrderField::from_str()` moved from `src/parse_tests.rs` to `tests/lib.rs`
  - Better organization with tests grouped by type in appropriate sections
  - All tests remain comprehensive and maintain full coverage

### Added
- **Display Trait for Core Types**: All core types now implement `Display` for user-facing output
  - `Parameter`: Formats as `similarity:value1,value2` (e.g., `contains:damian`)
  - `OrderField`: Formats as `name:order` (e.g., `date_created:desc`)
  - `Similarity`: Formats as similarity string (e.g., `contains`, `equals`)
  - `SortDirection`: Formats as direction string (e.g., `asc`, `desc`)
  - `Parameters`: Formats as HTTP query string (e.g., `name=contains:damian&age=between:20,30`)
  - `Order`: Formats as comma-separated order fields (e.g., `name:asc,date_created:desc`)
- **Comprehensive Display Tests**: Added tests for all `Display` implementations
  - Tests for `Parameter`, `OrderField`, `Similarity`, `SortDirection` display formatting
  - Tests for `Parameters` and `Order` display formatting with edge cases
  - Tests for empty collections, filtering behavior, and multiple values

### Breaking Changes
- **Type Renaming**: `SortOrder` enum renamed to `SortDirection`
  - `SortOrder` → `SortDirection` (enum name)
  - `OrderField::order()` → `OrderField::sort_direction()` (method name)
  - `Error::InvalidSortOrder` → `Error::InvalidSortDirection` (error variant)
  - Migration: Update all `SortOrder::` references to `SortDirection::`
  - Migration: Update `order_field.order()` to `order_field.sort_direction()`
  - Migration: Update error handling from `Error::InvalidSortOrder(msg)` to `Error::InvalidSortDirection(msg)`
  - Example: `SortOrder::Ascending` → `SortDirection::Ascending`, `order_field.order()` → `order_field.sort_direction()`
- **SQL Module Refactoring**: SQL types moved to `sql` module namespace
  - `SqlValue` → `sql::Value` (enum)
  - `"null"` string literal → `sql::NULL` constant
  - Migration: Update all `SqlValue::` references to `sql::Value::`
  - Migration: Update `"null"` string comparisons to use `sql::NULL`
  - Example: `SqlValue::Null` → `sql::Value::Null`, `if value == "null"` → `if value == sql::NULL`

### Technical Details
- **Rust Idioms**: `Display` is the standard trait for user-facing output in Rust
- **Automatic ToString**: `Display` implementation automatically provides `ToString` via blanket impl
- **Better Formatting**: Direct use in format strings: `format!("{param}")` instead of `format!("{}", param.to_string())`
- **Code Quality**: Improved separation of concerns with formatting logic in type implementations
- **Module Organization**: SQL-related types grouped in dedicated module for better code structure
- **Test Coverage**: All Display implementations are thoroughly tested with edge cases
- **Backward Compatibility**: All existing `.to_string()` calls continue to work unchanged
- **Version Bump**: Minor version bump reflects API improvements, new trait implementations, and module refactoring

## [0.9.0] - 2025-11-30

### Changed
- **Type Renaming**: Renamed `SortFields` struct to `Order` for better consistency and clarity
  - `SortFields` → `Order` (struct name)
  - `sort_fields` → `order` (field name in `Query` struct)
  - Matches the HTTP parameter name (`order=...`) for better API consistency
  - Shorter, more intuitive naming that communicates intent clearly
- **Error Type Renaming**: Renamed error variant to match `Order` naming
  - `Error::InvalidSortField` → `Error::InvalidOrderField`
  - Updated error message from "Invalid Sort Field" to "Invalid Order Field"
  - Maintains consistency with the `Order` type refactoring
- **Encapsulation**: Made tuple struct fields private for better encapsulation
  - `Order`, `Parameter`, and `Parameters` tuple struct fields are now private
  - Added `Parameter::init()` constructor method for creating `Parameter` instances
  - Improved API safety by preventing direct field access
- **Internal API**: Made internal utility functions and constants crate-private
  - `parse_parameter()` and `parse_order_field()` are now `pub(crate)` (internal-only)
  - Constants (`QUESTION`, `AMPERSAND`, `EQUAL`, `COLON`, `COMMA`, `PERCENT`) are now `pub(crate)` (internal-only)
  - `url_decode()` and `url_encode()` are now `pub(crate)` (internal-only)
  - These were never part of the documented public API and are now properly encapsulated
  - Tests for these internal functions have been moved to a separate test module within the crate
- **SQL Feature**: Made SQL generation opt-in instead of enabled by default
  - The `sql` feature is no longer included in default features
  - Users must explicitly enable SQL generation with `features = ["sql"]`
  - Reduces binary size for users who don't need SQL functionality
  - Core query parsing functionality remains available without the feature

### Added
- **Parameter Constructor**: Added `Parameter::init()` method for creating `Parameter` instances
  - Use `Parameter::init(similarity, values)` instead of `Parameter(similarity, values)`
  - Provides a clean API for parameter construction

### Breaking Changes
- **Type Renaming**: `SortFields` struct renamed to `Order`
  - Update code from `SortFields::new()` to `Order::new()`
  - Update code from `query.sort_fields` to `query.order`
  - Update code from `Query::init(..., sort_fields, ...)` to `Query::init(..., order, ...)`
  - All method calls remain the same (e.g., `query.order.ascending(...)`)
- **Error Type Renaming**: `Error::InvalidSortField` renamed to `Error::InvalidOrderField`
  - Update error handling from `Error::InvalidSortField(msg)` to `Error::InvalidOrderField(msg)`
  - Error message format changed from "Invalid Sort Field" to "Invalid Order Field"
  - Migration: Update all error pattern matches and error handling code
- **Encapsulation**: Tuple struct fields are now private
  - **Direct field access removed**: Can no longer use `.0` or `.1` to access tuple struct fields
  - **Parameter construction**: Use `Parameter::init(similarity, values)` instead of `Parameter(similarity, values)`
  - **Collection access**: Use `.inner()` and `.inner_mut()` methods to access underlying collections
  - **Migration guide**:
    - `query.parameters.0.get("name")` → `query.parameters.inner().get("name")`
    - `query.order.0.len()` → `query.order.inner().len()`
    - `Parameter(similarity, values)` → `Parameter::init(similarity, values)`
    - `param.0` → `*param.similarity()` or `param.similarity()`
    - `param.1` → `*param.values()` or `param.values()`
- **SQL Feature**: SQL generation is now opt-in
  - **Default behavior changed**: SQL generation methods are no longer available by default
  - **Migration**: Add `features = ["sql"]` to your `Cargo.toml` dependency if you use SQL generation
  - **Before**: `query-lite = "0.9.0"` (SQL enabled by default)
  - **After**: `query-lite = { version = "0.9.0", features = ["sql"] }` (SQL must be explicitly enabled)
  - **Affected methods**: `to_sql()`, `where_clause()`, `order_clause()`, `to_values()`, `parameter_values()`, `pagination_values()`, `total_parameters()`

### Technical Details
- **API Consistency**: Type name now matches the HTTP query parameter name (`order`)
- **Improved Readability**: `query.order.ascending(...)` reads more naturally than `query.sort_fields.ascending(...)`
- **One-Word Naming**: Simpler, more concise type name following Rust naming conventions
- **Better Encapsulation**: Private fields enforce use of accessor methods, improving API stability
- **Reduced API Surface**: Internal utilities are no longer exposed, reducing maintenance burden and preventing accidental usage
- **Test Organization**: Internal function tests moved to `src/parse_tests.rs` for better organization
- **Smaller Binary Size**: SQL feature being opt-in reduces default binary size for users who don't need SQL generation
- **Feature Flexibility**: Users can now choose to include only the functionality they need
- **Version Bump**: Minor version bump reflects breaking API changes

## [0.8.0] - 2025-01-27

### Changed
- **SQL Clause Methods**: Updated `where_clause()` and `order_clause()` to return `Option<String>`
  - `Query::where_clause()`: Now returns `Option<String>` instead of `String`
  - `Query::order_clause()`: Now returns `Option<String>` instead of `String`
  - Returns `None` when no conditions/sorting exist, `Some(clause)` otherwise
  - Prevents bugs from empty string handling and makes intent clearer

### Added
- **Improved SQL API**: `Option<String>` return types for SQL clause methods provide better semantic clarity
- **Better Error Prevention**: Eliminates accidental inclusion of empty clauses in SQL
- **Enhanced Type Safety**: Forces explicit handling of empty cases

### Breaking Changes
- **SQL Clause Methods**: `where_clause()` and `order_clause()` now return `Option<String>`
  - Update code from `let clause = query.where_clause(); if !clause.is_empty() { ... }`
  - To `if let Some(clause) = query.where_clause() { ... }`

### Technical Details
- **SQL Safety**: `Option<String>` prevents accidental inclusion of empty clauses in SQL
- **Rust Idiomatic**: Follows Rust best practices for "might not exist" scenarios
- **API Consistency**: Aligns with Rust's standard approach to optional values
- **Version Bump**: Major version bump reflects breaking API changes

## [0.7.0] - 2025-01-27

### Changed
- **Parameter Type Refactoring**: Converted `Parameter` from type alias to proper struct
  - `Parameter` is now a tuple struct `Parameter(Similarity, Vec<String>)` instead of a type alias
  - Added semantic access methods: `similarity()`, `values()`, and `values_mut()`
  - Maintains full backward compatibility with tuple field access (`.0`, `.1`)
  - Improved API design with clearer intent and better extensibility

### Added
- **Enhanced Parameter Access**: New methods for semantic parameter data access
  - `Parameter::similarity()`: Get immutable reference to similarity type
  - `Parameter::values()`: Get immutable reference to parameter values  
  - `Parameter::values_mut()`: Get mutable reference to parameter values
- **Better API Design**: Struct-based approach provides clearer intent than generic tuple type alias
- **Future Extensibility**: Easier to add new methods or fields to Parameter struct

### Technical Details
- **Backward Compatibility**: All existing tuple access patterns continue to work unchanged
- **Zero Breaking Changes**: Existing code using `.0` and `.1` field access remains functional
- **Improved Type Safety**: More explicit about what the type represents
- **Comprehensive Testing**: Updated 100+ test cases to use new struct syntax
- **Version Bump**: Major version bump reflects significant internal API improvements

## [0.6.1] - 2025-10-01

### Added
- **Code Refactoring**: Eliminated code duplication in SQL value methods
  - `Query::to_values()` now internally uses `parameter_values()` and `pagination_values()`
  - Reduced code duplication from 40+ lines to 3 lines
  - Improved maintainability through composition over duplication

### Technical Details
- **DRY Principle**: Eliminated code duplication through method composition
- **Single Responsibility**: Each method now has a clear, focused purpose
- **Maintainability**: Changes to parameter value logic only need to be made in one place
- **Performance**: No performance impact, same functionality with cleaner code

## [0.6.0] - 2025-10-01

### Changed
- **API Simplification**: Simplified SQL value methods for better usability
  - `Query::to_values()`: Now returns `Vec<SqlValue>` directly instead of `Result<Vec<SqlValue>>`
  - `Query::parameter_values()`: Now returns `Vec<SqlValue>` directly instead of `Result<Vec<SqlValue>>`
- **Code Quality**: Improved internal implementation and error handling
  - Removed unreachable `InvalidSqlValue` error type
  - Simplified iteration patterns for better consistency
  - Eliminated impossible error conditions

### Technical Details
- **Breaking Changes**: Method return types changed from `Result<Vec<SqlValue>>` to `Vec<SqlValue>`
- **Performance**: Removed unnecessary error handling overhead
- **Consistency**: Both SQL value methods now use identical iteration patterns
- **Maintainability**: Cleaner, more readable code with consistent error handling patterns

## [0.5.0] - 2025-10-01

### Changed
- **API Consistency**: Renamed SQL-related methods for better consistency and clarity
  - `Query::to_parameter_values()` → `Query::parameter_values()`
  - `Query::to_pagination_values()` → `Query::pagination_values()`
  - `Query::build_where_clause()` → `Query::where_clause()` (now public)
  - `Query::build_order_clause()` → `Query::order_clause()` (now public)
- **Package Rename**: Renamed package from `query-x` to `query-lite` for better clarity
- **Documentation**: Updated all documentation to reflect new package name and function names

### Technical Details
- **Breaking Changes**: Function renames are breaking changes, hence the major version bump
- **Backward Compatibility**: Old function names are no longer available
- **Public API**: Made `where_clause()` and `order_clause()` public for better API flexibility
- **Consistent Naming**: All SQL-related methods now follow consistent naming patterns

## [0.4.0] - 2025-12-19

### Added
- **Granular SQL Value Management**: New methods for fine-grained control over SQL parameter values
  - `Query::parameter_values()`: Get SQL values for parameters only (without limit and offset)
  - `Query::pagination_values()`: Get SQL values for pagination (limit and offset only)
  - `Query::total_parameters()`: Get the total number of SQL parameter values
- **Enhanced SQL Workflow**: Improved SQL generation workflow with separate parameter and pagination handling
- **Performance Optimization**: Ability to process parameter values and pagination values independently
- **Better Debugging**: Easier inspection of parameter counts and values for troubleshooting

### Technical Details
- **Backward Compatibility**: All existing SQL functionality continues to work without changes
- **Feature-Gated**: New methods are only available when the `sql` feature is enabled
- **Zero-Cost Abstractions**: New methods provide additional functionality without performance overhead
- **Flexible Integration**: Supports various database driver patterns and custom value processing

## [0.3.1] - 2025-12-19

### Added
- **Error Comparison**: Added `PartialEq` derive to `Error` enum for better testing and comparison capabilities

## [0.3.0] - 2025-12-19

### Added
- **Parameter Type Alias and Trait**: New `Parameter` type alias and `ParameterGet` trait for semantic parameter access
- **Enhanced Parameter Access**: Added `similarity()` and `values()` methods for clearer parameter data access
- **Direct Collection Access**: Added `inner()` and `inner_mut()` methods to `Parameters` and `SortFields` for advanced operations
- **Builder Pattern**: Enhanced programmatic query construction with fluent API methods
- **Comprehensive Test Suite**: Added 67 new tests covering all new features and edge cases

### Changed
- **Method Renaming**: Renamed `SortFields::asc()`/`desc()` to `ascending()`/`descending()` for better clarity
- **API Ergonomics**: Improved parameter access patterns while maintaining full backward compatibility
- **HTTP Generation**: Fixed missing `order=` prefix in `to_http()` method for proper roundtrip compatibility
- **Type Safety**: Enhanced type safety with trait-based parameter access

### Fixed
- **HTTP Roundtrip**: Fixed bug where sort fields were missing `order=` prefix in generated HTTP strings
- **Parameter Access**: Improved consistency between tuple access and trait-based access methods

### Technical Details
- **Backward Compatibility**: All existing code continues to work without changes
- **Performance**: Zero-cost abstractions with trait methods that compile to direct field access
- **Flexibility**: Multiple access patterns for different use cases (basic, semantic, advanced)
- **Future-Proof**: Trait-based design allows for easy API evolution without breaking changes

## [0.1.0] - 2025-12-19

### Added
- Initial release
- Support for traditional and advanced query parameters
- SQL generation with parameter binding
- Comprehensive test suite
- Full documentation
