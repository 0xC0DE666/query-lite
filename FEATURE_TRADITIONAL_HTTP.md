# Feature: Traditional HTTP Query Format Support

## Overview

Add support for generating traditional-style HTTP query strings (e.g., `?name=john&age=25`) in addition to the existing advanced format (e.g., `?name=equals:john&age=equals:25`).

## Motivation

Some use cases require simple, traditional HTTP query strings without similarity prefixes. This feature allows users to choose between:
- **Advanced format**: `name=equals:john&age=between:20,30` (current behavior)
- **Traditional format**: `name=john&age=25` (new behavior)

## Design Decisions

### 1. Configuration Approach

Use a field on the `Query` struct to control output format:

```rust
pub struct Query {
    pub parameters: Parameters,
    pub order: Order,
    pub limit: usize,
    pub offset: usize,
    pub http_format: HttpFormat,  // New field
}

pub enum HttpFormat {
    Advanced,      // Current behavior: includes similarity prefixes
    Traditional,   // New behavior: simple key=value pairs (only for Equals)
}
```

### 2. Traditional Format Behavior

- **Only include parameters with `Similarity::Equals`**: Skip all other similarities (Contains, Between, Greater, etc.)
- **No similarity prefix**: Output `name=john` instead of `name=equals:john`
- **Multiple values**: Handle comma-separated values: `name=john,jane` (traditional style)
- **Order parameter**: Keep current format `order=field:asc,field2:desc` (traditional HTTP doesn't have a standard for sorting)
- **Limit/Offset**: Include as-is: `limit=50&offset=0`

### 3. Implementation Structure

Split `to_http()` into internal methods:

```rust
impl Query {
    pub fn to_http(&self) -> String {
        match self.http_format {
            HttpFormat::Advanced => self.to_http_advanced(),
            HttpFormat::Traditional => self.to_http_traditional(),
        }
    }
    
    pub(crate) fn to_http_advanced(&self) -> String {
        // Current implementation (moved from to_http)
    }
    
    pub(crate) fn to_http_traditional(&self) -> String {
        // New implementation
    }
}
```

**Rationale for `pub(crate)`:**
- Allows internal testing of both formats independently
- Keeps the public API clean with a single `to_http()` method
- Enables unit tests to verify each format's behavior

## Implementation Details

### Step 1: Add HttpFormat Enum

```rust
#[derive(Clone, Debug, PartialEq)]
pub enum HttpFormat {
    Advanced,
    Traditional,
}

impl Default for HttpFormat {
    fn default() -> Self {
        Self::Advanced  // Maintain backward compatibility
    }
}
```

### Step 2: Add Field to Query Struct

```rust
pub struct Query {
    pub parameters: Parameters,
    pub order: Order,
    pub limit: usize,
    pub offset: usize,
    pub http_format: HttpFormat,  // Add this
}
```

### Step 3: Update Query::new()

```rust
pub fn new() -> Self {
    Self {
        parameters: Parameters::new(),
        order: Order::new(),
        limit: Parameters::DEFAULT_LIMIT,
        offset: Parameters::DEFAULT_OFFSET,
        http_format: HttpFormat::default(),  // Defaults to Advanced
    }
}
```

### Step 4: Update Query::init()

```rust
pub fn init(parameters: Parameters, order: Order, limit: usize, offset: usize) -> Self {
    Self {
        parameters,
        order,
        limit,
        offset,
        http_format: HttpFormat::default(),  // Defaults to Advanced
    }
}

// Or add an overloaded version:
pub fn init_with_format(
    parameters: Parameters, 
    order: Order, 
    limit: usize, 
    offset: usize,
    http_format: HttpFormat
) -> Self {
    Self {
        parameters,
        order,
        limit,
        offset,
        http_format,
    }
}
```

### Step 5: Refactor to_http()

```rust
pub fn to_http(&self) -> String {
    match self.http_format {
        HttpFormat::Advanced => self.to_http_advanced(),
        HttpFormat::Traditional => self.to_http_traditional(),
    }
}
```

### Step 6: Extract Current Implementation

Move current `to_http()` implementation to `to_http_advanced()`:

```rust
pub(crate) fn to_http_advanced(&self) -> String {
    // Current implementation from to_http()
    // - Includes similarity prefixes
    // - Includes all similarity types
    // - Format: name=equals:john, name=contains:john, etc.
}
```

### Step 7: Implement to_http_traditional()

```rust
pub(crate) fn to_http_traditional(&self) -> String {
    let mut params_str = self
        .parameters
        .inner()
        .iter()
        .filter(|(_, param)| {
            // Only include Equals similarity
            *param.similarity() == Similarity::Equals && param.values().len() > 0
        })
        .map(|(key, param)| {
            let values_str = param
                .values()
                .iter()
                .map(|v| url_encode(v))
                .collect::<Vec<String>>()
                .join(&format!("{COMMA}"));
            format!("{key}{EQUAL}{values_str}")
        })
        .collect::<Vec<String>>()
        .join("&");

    // Handle order parameter (keep current format)
    let order_str = self
        .order
        .inner()
        .iter()
        .filter(|(name, _)| name.len() > 0)
        .map(|(name, sort_order)| format!("{name}{COLON}{}", sort_order.to_string()))
        .collect::<Vec<String>>()
        .join(&format!("{COMMA}"));

    if params_str.len() > 0 {
        params_str.push_str(&format!("{AMPERSAND}"));
    }

    if order_str.len() > 0 {
        params_str.push_str(&format!("{}{EQUAL}{}", Parameters::ORDER, order_str));
        params_str.push_str(&format!("{AMPERSAND}"));
    }

    format!(
        "{params_str}{}{EQUAL}{}{AMPERSAND}{}{EQUAL}{}",
        Parameters::LIMIT,
        self.limit,
        Parameters::OFFSET,
        self.offset,
    )
}
```

## Edge Cases to Handle

### 1. Empty Parameters
- If no Equals parameters exist, output should still include `limit` and `offset`
- Result: `limit=50&offset=0`

### 2. Multiple Values for Equals
- Traditional format: `name=john,jane` (comma-separated)
- This matches how traditional HTTP queries handle multiple values

### 3. URL Encoding
- Must still URL-encode values in traditional format
- Use existing `url_encode()` function

### 4. Order Parameter
- Keep current format: `order=field:asc,field2:desc`
- Traditional HTTP doesn't have a standard for sorting, so this is acceptable

### 5. Limit/Offset
- Always include, even if they're default values
- Format: `limit=50&offset=0`

### 6. Mixed Similarities
- If query has both Equals and Contains parameters:
  - Advanced format: includes both
  - Traditional format: only includes Equals, skips Contains

## Testing Requirements

### Unit Tests for to_http_advanced()
- Test that current behavior is preserved
- Test all similarity types are included
- Test multiple values
- Test order parameter
- Test limit/offset

### Unit Tests for to_http_traditional()
- Test only Equals parameters are included
- Test non-Equals parameters are excluded
- Test multiple values (comma-separated)
- Test order parameter format
- Test limit/offset inclusion
- Test empty parameters case
- Test URL encoding

### Integration Tests
- Test roundtrip: traditional → parse → traditional
- Test roundtrip: advanced → parse → advanced
- Test format switching
- Test from_http() with traditional format input

## API Changes

### Breaking Changes
- **None**: `HttpFormat` defaults to `Advanced`, maintaining backward compatibility

### New Public API
- `HttpFormat` enum (public)
- `Query::http_format` field (public)
- Optional: `Query::init_with_format()` method

### Internal API
- `Query::to_http_advanced()` - `pub(crate)` for testing
- `Query::to_http_traditional()` - `pub(crate)` for testing

## Migration Guide

### For Users

**No migration needed** - existing code continues to work:

```rust
let query = Query::from_http("name=john".to_string())?;
let http = query.to_http();  // Still works, uses Advanced format
```

**To use Traditional format:**

```rust
let mut query = Query::from_http("name=john".to_string())?;
query.http_format = HttpFormat::Traditional;
let http = query.to_http();  // Now uses Traditional format
```

**Or create with format:**

```rust
let query = Query::init_with_format(
    parameters,
    order,
    limit,
    offset,
    HttpFormat::Traditional
);
```

## Example Outputs

### Advanced Format (Current)
```
name=equals:john&age=equals:25&category=contains:electronics&order=date:desc&limit=50&offset=0
```

### Traditional Format (New)
```
name=john&age=25&order=date:desc&limit=50&offset=0
```
Note: `category=contains:electronics` is excluded because it's not Equals similarity.

## Future Considerations

1. **Builder method**: Consider adding `Query::with_format()` for fluent API
2. **Per-parameter format**: Could allow mixing formats (probably overkill)
3. **Format detection**: Could auto-detect format based on query content
4. **Documentation**: Update README with examples of both formats

## Implementation Checklist

- [ ] Add `HttpFormat` enum
- [ ] Add `http_format` field to `Query` struct
- [ ] Update `Query::new()` to set default format
- [ ] Update `Query::init()` to set default format
- [ ] Optionally add `Query::init_with_format()`
- [ ] Refactor `to_http()` to use match on format
- [ ] Extract current implementation to `to_http_advanced()`
- [ ] Implement `to_http_traditional()`
- [ ] Add unit tests for `to_http_advanced()`
- [ ] Add unit tests for `to_http_traditional()`
- [ ] Add integration tests for format switching
- [ ] Update documentation (README.md)
- [ ] Update CHANGELOG.md
- [ ] Consider adding builder method for fluent API

