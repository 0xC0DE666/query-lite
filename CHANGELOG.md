# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1] - 2024-12-19

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

## [0.1.0] - 2024-12-19

### Added
- Initial release
- Support for traditional and advanced query parameters
- SQL generation with parameter binding
- Comprehensive test suite
- Full documentation
