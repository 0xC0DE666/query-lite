# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
