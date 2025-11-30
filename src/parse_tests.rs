use crate::error::Error;
use crate::*;

// ============================================================================
// PARSE FUNCTION TESTS
// ============================================================================

// Tests for parse_parameter function
#[test]
fn test_parse_parameter_contains() {
    let param = parse_parameter("contains:damian").unwrap();
    assert_eq!(*param.similarity(), Similarity::Contains);
    assert_eq!(*param.values(), vec!["damian"]);
}

#[test]
fn test_parse_parameter_equals_multiple() {
    let param = parse_parameter("equals:black,steel,wood").unwrap();
    assert_eq!(*param.similarity(), Similarity::Equals);
    assert_eq!(*param.values(), vec!["black", "steel", "wood"]);
}

#[test]
fn test_parse_parameter_between() {
    let param = parse_parameter("between:20,30").unwrap();
    assert_eq!(*param.similarity(), Similarity::Between);
    assert_eq!(*param.values(), vec!["20", "30"]);
}

#[test]
fn test_parse_parameter_lesser() {
    let param = parse_parameter("lesser:100").unwrap();
    assert_eq!(*param.similarity(), Similarity::Lesser);
    assert_eq!(*param.values(), vec!["100"]);
}

#[test]
fn test_parse_parameter_greater_or_equal() {
    let param = parse_parameter("greater-or-equal:50").unwrap();
    assert_eq!(*param.similarity(), Similarity::GreaterOrEqual);
    assert_eq!(*param.values(), vec!["50"]);
}

#[test]
fn test_parse_parameter_with_whitespace() {
    let param = parse_parameter("  contains  :  damian  ").unwrap();
    assert_eq!(*param.similarity(), Similarity::Contains);
    assert_eq!(*param.values(), vec!["damian"]);
}

#[test]
fn test_parse_parameter_with_whitespace_in_values() {
    let param = parse_parameter("equals: black , steel , wood ").unwrap();
    assert_eq!(*param.similarity(), Similarity::Equals);
    assert_eq!(*param.values(), vec!["black", "steel", "wood"]);
}

#[test]
fn test_parse_parameter_empty_values() {
    let param = parse_parameter("contains:").unwrap();
    assert_eq!(*param.similarity(), Similarity::Contains);
    assert_eq!(*param.values(), vec![] as Vec<String>);
}

#[test]
fn test_parse_parameter_empty_values_with_commas() {
    let param = parse_parameter("contains:,,,").unwrap();
    assert_eq!(*param.similarity(), Similarity::Contains);
    assert_eq!(*param.values(), vec![] as Vec<String>);
}

#[test]
fn test_parse_parameter_mixed_empty_values() {
    let param = parse_parameter("contains:value1,,value2,").unwrap();
    assert_eq!(*param.similarity(), Similarity::Contains);
    assert_eq!(*param.values(), vec!["value1", "value2"]);
}

#[test]
fn test_parse_parameter_invalid_empty() {
    assert!(parse_parameter("").is_err());
}

#[test]
fn test_parse_parameter_invalid_no_colon() {
    assert!(parse_parameter("contains").is_err());
    assert!(parse_parameter("containsdamian").is_err());
}

#[test]
fn test_parse_parameter_invalid_multiple_colons() {
    assert!(parse_parameter("contains:damian:extra").is_err());
}

#[test]
fn test_parse_parameter_invalid_empty_similarity() {
    assert!(parse_parameter(":damian").is_err());
}

#[test]
fn test_parse_parameter_invalid_similarity() {
    assert!(parse_parameter("invalid:damian").is_err());
}

#[test]
fn test_parse_parameter_invalid_whitespace_only() {
    assert!(parse_parameter("   ").is_err());
}

// Tests for parse_sort_field function
#[test]
fn test_parse_sort_field_asc() {
    let (name, order) = parse_sort_field("name:asc").unwrap();
    assert_eq!(name, "name");
    assert_eq!(order, SortOrder::Ascending);
}

#[test]
fn test_parse_sort_field_desc() {
    let (name, order) = parse_sort_field("date_created:desc").unwrap();
    assert_eq!(name, "date_created");
    assert_eq!(order, SortOrder::Descending);
}

#[test]
fn test_parse_sort_field_with_whitespace() {
    let (name, order) = parse_sort_field("  name  :  asc  ").unwrap();
    assert_eq!(name, "name");
    assert_eq!(order, SortOrder::Ascending);
}

#[test]
fn test_parse_sort_field_with_special_characters() {
    let (name, order) = parse_sort_field("user_name:desc").unwrap();
    assert_eq!(name, "user_name");
    assert_eq!(order, SortOrder::Descending);
}

#[test]
fn test_parse_sort_field_invalid_empty() {
    assert!(parse_sort_field("").is_err());
}

#[test]
fn test_parse_sort_field_invalid_no_colon() {
    assert!(parse_sort_field("name").is_err());
    assert!(parse_sort_field("nameasc").is_err());
}

#[test]
fn test_parse_sort_field_invalid_multiple_colons() {
    assert!(parse_sort_field("name:asc:extra").is_err());
}

#[test]
fn test_parse_sort_field_invalid_empty_name() {
    assert!(parse_sort_field(":asc").is_err());
}

#[test]
fn test_parse_sort_field_invalid_empty_order() {
    assert!(parse_sort_field("name:").is_err());
}

#[test]
fn test_parse_sort_field_invalid_order() {
    assert!(parse_sort_field("name:invalid").is_err());
}

#[test]
fn test_parse_sort_field_invalid_whitespace_only() {
    assert!(parse_sort_field("   ").is_err());
}

#[test]
fn test_error_invalid_parameter() {
    let error = parse_parameter("invalid").unwrap_err();
    match error {
        Error::InvalidParameter(msg) => assert_eq!(msg, "invalid"),
        _ => panic!("Expected InvalidParameter error"),
    }
}

// Tests for constants
#[test]
fn test_constants() {
    assert_eq!(QUESTION, '?');
    assert_eq!(AMPERSAND, '&');
    assert_eq!(EQUAL, '=');
    assert_eq!(COLON, ':');
    assert_eq!(COMMA, ',');
    assert_eq!(PERCENT, '%');
}

