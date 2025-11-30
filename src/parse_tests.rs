use crate::error::Error;
use crate::*;

// ============================================================================
// FROMSTR IMPLEMENTATION TESTS
// ============================================================================

// Tests for Parameter::from_str
#[test]
fn test_parse_parameter_contains() {
    let param = "contains:damian".parse::<Parameter>().unwrap();
    assert_eq!(*param.similarity(), Similarity::Contains);
    assert_eq!(*param.values(), vec!["damian"]);
}

#[test]
fn test_parse_parameter_equals_multiple() {
    let param = "equals:black,steel,wood".parse::<Parameter>().unwrap();
    assert_eq!(*param.similarity(), Similarity::Equals);
    assert_eq!(*param.values(), vec!["black", "steel", "wood"]);
}

#[test]
fn test_parse_parameter_between() {
    let param = "between:20,30".parse::<Parameter>().unwrap();
    assert_eq!(*param.similarity(), Similarity::Between);
    assert_eq!(*param.values(), vec!["20", "30"]);
}

#[test]
fn test_parse_parameter_lesser() {
    let param = "lesser:100".parse::<Parameter>().unwrap();
    assert_eq!(*param.similarity(), Similarity::Lesser);
    assert_eq!(*param.values(), vec!["100"]);
}

#[test]
fn test_parse_parameter_greater_or_equal() {
    let param = "greater-or-equal:50".parse::<Parameter>().unwrap();
    assert_eq!(*param.similarity(), Similarity::GreaterOrEqual);
    assert_eq!(*param.values(), vec!["50"]);
}

#[test]
fn test_parse_parameter_with_whitespace() {
    let param = "  contains  :  damian  ".parse::<Parameter>().unwrap();
    assert_eq!(*param.similarity(), Similarity::Contains);
    assert_eq!(*param.values(), vec!["damian"]);
}

#[test]
fn test_parse_parameter_with_whitespace_in_values() {
    let param = "equals: black , steel , wood ".parse::<Parameter>().unwrap();
    assert_eq!(*param.similarity(), Similarity::Equals);
    assert_eq!(*param.values(), vec!["black", "steel", "wood"]);
}

#[test]
fn test_parse_parameter_empty_values() {
    let param = "contains:".parse::<Parameter>().unwrap();
    assert_eq!(*param.similarity(), Similarity::Contains);
    assert_eq!(*param.values(), vec![] as Vec<String>);
}

#[test]
fn test_parse_parameter_empty_values_with_commas() {
    let param = "contains:,,,".parse::<Parameter>().unwrap();
    assert_eq!(*param.similarity(), Similarity::Contains);
    assert_eq!(*param.values(), vec![] as Vec<String>);
}

#[test]
fn test_parse_parameter_mixed_empty_values() {
    let param = "contains:value1,,value2,".parse::<Parameter>().unwrap();
    assert_eq!(*param.similarity(), Similarity::Contains);
    assert_eq!(*param.values(), vec!["value1", "value2"]);
}

#[test]
fn test_parse_parameter_invalid_empty() {
    assert!("".parse::<Parameter>().is_err());
}

#[test]
fn test_parse_parameter_invalid_no_colon() {
    assert!("contains".parse::<Parameter>().is_err());
    assert!("containsdamian".parse::<Parameter>().is_err());
}

#[test]
fn test_parse_parameter_invalid_multiple_colons() {
    assert!("contains:damian:extra".parse::<Parameter>().is_err());
}

#[test]
fn test_parse_parameter_invalid_empty_similarity() {
    assert!(":damian".parse::<Parameter>().is_err());
}

#[test]
fn test_parse_parameter_invalid_similarity() {
    assert!("invalid:damian".parse::<Parameter>().is_err());
}

#[test]
fn test_parse_parameter_invalid_whitespace_only() {
    assert!("   ".parse::<Parameter>().is_err());
}

// Tests for OrderField::from_str
#[test]
fn test_parse_order_field_asc() {
    let order_field = "name:asc".parse::<OrderField>().unwrap();
    assert_eq!(order_field.name(), "name");
    assert_eq!(*order_field.order(), SortOrder::Ascending);
}

#[test]
fn test_parse_order_field_desc() {
    let order_field = "date_created:desc".parse::<OrderField>().unwrap();
    assert_eq!(order_field.name(), "date_created");
    assert_eq!(*order_field.order(), SortOrder::Descending);
}

#[test]
fn test_parse_order_field_with_whitespace() {
    let order_field = "  name  :  asc  ".parse::<OrderField>().unwrap();
    assert_eq!(order_field.name(), "name");
    assert_eq!(*order_field.order(), SortOrder::Ascending);
}

#[test]
fn test_parse_order_field_with_special_characters() {
    let order_field = "user_name:desc".parse::<OrderField>().unwrap();
    assert_eq!(order_field.name(), "user_name");
    assert_eq!(*order_field.order(), SortOrder::Descending);
}

#[test]
fn test_parse_order_field_invalid_empty() {
    assert!("".parse::<OrderField>().is_err());
}

#[test]
fn test_parse_order_field_invalid_no_colon() {
    assert!("name".parse::<OrderField>().is_err());
    assert!("nameasc".parse::<OrderField>().is_err());
}

#[test]
fn test_parse_order_field_invalid_multiple_colons() {
    assert!("name:asc:extra".parse::<OrderField>().is_err());
}

#[test]
fn test_parse_order_field_invalid_empty_name() {
    assert!(":asc".parse::<OrderField>().is_err());
}

#[test]
fn test_parse_order_field_invalid_empty_order() {
    assert!("name:".parse::<OrderField>().is_err());
}

#[test]
fn test_parse_order_field_invalid_order() {
    assert!("name:invalid".parse::<OrderField>().is_err());
}

#[test]
fn test_parse_order_field_invalid_whitespace_only() {
    assert!("   ".parse::<OrderField>().is_err());
}

#[test]
fn test_error_invalid_parameter() {
    let error = "invalid".parse::<Parameter>().unwrap_err();
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

