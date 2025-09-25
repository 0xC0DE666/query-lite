use std::str::FromStr;
use query_x::error::Error;
use query_x::*;

// ============================================================================
// PARSE FUNCTION TESTS
// ============================================================================

// Tests for parse_parameter function
#[test]
fn test_parse_parameter_contains() {
    let (similarity, values) = parse_parameter("contains:damian").unwrap();
    assert_eq!(similarity, Similarity::Contains);
    assert_eq!(values, vec!["damian"]);
}

#[test]
fn test_parse_parameter_equals_multiple() {
    let (similarity, values) = parse_parameter("equals:black,steel,wood").unwrap();
    assert_eq!(similarity, Similarity::Equals);
    assert_eq!(values, vec!["black", "steel", "wood"]);
}

#[test]
fn test_parse_parameter_between() {
    let (similarity, values) = parse_parameter("between:20,30").unwrap();
    assert_eq!(similarity, Similarity::Between);
    assert_eq!(values, vec!["20", "30"]);
}

#[test]
fn test_parse_parameter_lesser() {
    let (similarity, values) = parse_parameter("lesser:100").unwrap();
    assert_eq!(similarity, Similarity::Lesser);
    assert_eq!(values, vec!["100"]);
}

#[test]
fn test_parse_parameter_greater_or_equal() {
    let (similarity, values) = parse_parameter("greater-or-equal:50").unwrap();
    assert_eq!(similarity, Similarity::GreaterOrEqual);
    assert_eq!(values, vec!["50"]);
}

#[test]
fn test_parse_parameter_with_whitespace() {
    let (similarity, values) = parse_parameter("  contains  :  damian  ").unwrap();
    assert_eq!(similarity, Similarity::Contains);
    assert_eq!(values, vec!["damian"]);
}

#[test]
fn test_parse_parameter_with_whitespace_in_values() {
    let (similarity, values) = parse_parameter("equals: black , steel , wood ").unwrap();
    assert_eq!(similarity, Similarity::Equals);
    assert_eq!(values, vec!["black", "steel", "wood"]);
}

#[test]
fn test_parse_parameter_empty_values() {
    let (similarity, values) = parse_parameter("contains:").unwrap();
    assert_eq!(similarity, Similarity::Contains);
    assert_eq!(values, vec![] as Vec<String>);
}

#[test]
fn test_parse_parameter_empty_values_with_commas() {
    let (similarity, values) = parse_parameter("contains:,,,").unwrap();
    assert_eq!(similarity, Similarity::Contains);
    assert_eq!(values, vec![] as Vec<String>);
}

#[test]
fn test_parse_parameter_mixed_empty_values() {
    let (similarity, values) = parse_parameter("contains:value1,,value2,").unwrap();
    assert_eq!(similarity, Similarity::Contains);
    assert_eq!(values, vec!["value1", "value2"]);
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

// ============================================================================
// CONSTANTS TESTS
// ============================================================================

#[test]
fn test_constants() {
    assert_eq!(QUESTION, '?');
    assert_eq!(AMPERSAND, '&');
    assert_eq!(EQUAL, '=');
    assert_eq!(COLON, ':');
    assert_eq!(COMMA, ',');
}

// ============================================================================
// SORT ORDER TESTS
// ============================================================================

#[test]
fn test_sort_order_variants() {
    assert_eq!(SortOrder::Ascending, SortOrder::Ascending);
    assert_eq!(SortOrder::Descending, SortOrder::Descending);
    assert_ne!(SortOrder::Ascending, SortOrder::Descending);
}

#[test]
fn test_sort_order_constants() {
    assert_eq!(SortOrder::ASCENDING, "asc");
    assert_eq!(SortOrder::DESCENDING, "desc");
}

#[test]
fn test_sort_order_default() {
    assert_eq!(SortOrder::default(), SortOrder::Ascending);
}

#[test]
fn test_sort_order_from_str_valid() {
    assert_eq!(SortOrder::from_str("asc").unwrap(), SortOrder::Ascending);
    assert_eq!(SortOrder::from_str("desc").unwrap(), SortOrder::Descending);
}

#[test]
fn test_sort_order_from_str_invalid() {
    assert!(SortOrder::from_str("invalid").is_err());
    assert!(SortOrder::from_str("").is_err());
    assert!(SortOrder::from_str("ASC").is_err());
    assert!(SortOrder::from_str("DESC").is_err());
    assert!(SortOrder::from_str("ascending").is_err());
    assert!(SortOrder::from_str("descending").is_err());
}

#[test]
fn test_sort_order_to_string() {
    assert_eq!(SortOrder::Ascending.to_string(), "asc");
    assert_eq!(SortOrder::Descending.to_string(), "desc");
}

// ============================================================================
// SORT FIELDS HELPER FUNCTIONS TESTS
// ============================================================================

#[test]
fn test_sort_fields_helper_functions_asc() {
    let mut fields = SortFields::new();
    fields.asc("name".to_string());
    
    assert_eq!(fields.0.len(), 1);
    assert_eq!(fields.0.get("name"), Some(&SortOrder::Ascending));
}

#[test]
fn test_sort_fields_helper_functions_desc() {
    let mut fields = SortFields::new();
    fields.desc("date_created".to_string());
    
    assert_eq!(fields.0.len(), 1);
    assert_eq!(fields.0.get("date_created"), Some(&SortOrder::Descending));
}

#[test]
fn test_sort_fields_helper_functions_fluent_api() {
    let mut fields = SortFields::new();
    fields.asc("name".to_string())
          .desc("date_created".to_string())
          .asc("email".to_string());
    
    assert_eq!(fields.0.len(), 3);
    assert_eq!(fields.0.get("name"), Some(&SortOrder::Ascending));
    assert_eq!(fields.0.get("date_created"), Some(&SortOrder::Descending));
    assert_eq!(fields.0.get("email"), Some(&SortOrder::Ascending));
}

#[test]
fn test_sort_fields_helper_functions_overwrite() {
    let mut fields = SortFields::new();
    fields.asc("name".to_string());
    fields.desc("name".to_string()); // Should overwrite
    
    assert_eq!(fields.0.len(), 1);
    assert_eq!(fields.0.get("name"), Some(&SortOrder::Descending));
}

#[test]
fn test_sort_fields_helper_functions_empty_name() {
    let mut fields = SortFields::new();
    fields.asc("".to_string());
    
    assert_eq!(fields.0.len(), 1);
    assert_eq!(fields.0.get(""), Some(&SortOrder::Ascending));
}

#[test]
fn test_sort_fields_helper_functions_whitespace_name() {
    let mut fields = SortFields::new();
    fields.asc("  name  ".to_string());
    
    assert_eq!(fields.0.len(), 1);
    assert_eq!(fields.0.get("  name  "), Some(&SortOrder::Ascending));
}

#[test]
fn test_sort_fields_helper_functions_special_characters() {
    let mut fields = SortFields::new();
    fields.asc("user_name_123".to_string());
    
    assert_eq!(fields.0.len(), 1);
    assert_eq!(fields.0.get("user_name_123"), Some(&SortOrder::Ascending));
}

#[test]
fn test_sort_fields_helper_functions_unicode_name() {
    let mut fields = SortFields::new();
    fields.asc("用户_姓名".to_string());
    
    assert_eq!(fields.0.len(), 1);
    assert_eq!(fields.0.get("用户_姓名"), Some(&SortOrder::Ascending));
}

#[test]
fn test_sort_fields_keep() {
    let mut fields = SortFields::new();
    fields.asc("name".to_string());
    fields.desc("date_created".to_string());
    fields.asc("email".to_string());

    let filtered = fields.keep(vec!["name".to_string(), "email".to_string()]);

    assert_eq!(filtered.0.len(), 2);
    assert_eq!(filtered.0.get("name"), Some(&SortOrder::Ascending));
    assert_eq!(filtered.0.get("date_created"), None);
    assert_eq!(filtered.0.get("email"), Some(&SortOrder::Ascending));
}

#[test]
fn test_sort_fields_keep_nonexistent_keys() {
    let mut fields = SortFields::new();
    fields.asc("name".to_string());

    let filtered = fields.keep(vec!["nonexistent".to_string()]);
    assert_eq!(filtered.0.len(), 0);
}

#[test]
fn test_sort_fields_keep_empty_keys() {
    let mut fields = SortFields::new();
    fields.asc("name".to_string());

    let filtered = fields.keep(vec![]);
    assert_eq!(filtered.0.len(), 0);
}

#[test]
fn test_sort_fields_remove() {
    let mut fields = SortFields::new();
    fields.asc("name".to_string());
    fields.desc("date_created".to_string());
    fields.asc("email".to_string());

    let filtered = fields.remove(vec!["name".to_string(), "email".to_string()]);

    assert_eq!(filtered.0.len(), 1);
    assert_eq!(filtered.0.get("name"), None);
    assert_eq!(filtered.0.get("date_created"), Some(&SortOrder::Descending));
    assert_eq!(filtered.0.get("email"), None);
}

#[test]
fn test_sort_fields_remove_nonexistent_keys() {
    let mut fields = SortFields::new();
    fields.asc("name".to_string());

    let filtered = fields.remove(vec!["nonexistent".to_string()]);
    assert_eq!(filtered.0.len(), 1);
    assert_eq!(filtered.0.get("name"), Some(&SortOrder::Ascending));
}

#[test]
fn test_sort_fields_remove_empty_keys() {
    let mut fields = SortFields::new();
    fields.asc("name".to_string());

    let filtered = fields.remove(vec![]);
    assert_eq!(filtered.0.len(), 1);
    assert_eq!(filtered.0.get("name"), Some(&SortOrder::Ascending));
}

// ============================================================================
// SORT FIELDS TESTS
// ============================================================================

#[test]
fn test_sort_fields_new() {
    let fields = SortFields::new();
    assert_eq!(fields.0.len(), 0);
}

#[test]
fn test_sort_fields_default() {
    let fields = SortFields::default();
    assert_eq!(fields.0.len(), 0);
}

#[test]
fn test_sort_fields_from_str_empty() {
    let fields = SortFields::from_str("").unwrap();
    assert_eq!(fields.0.len(), 0);

    let fields = SortFields::from_str("   ").unwrap();
    assert_eq!(fields.0.len(), 0);
}

#[test]
fn test_sort_fields_from_str_single() {
    let fields = SortFields::from_str("name:asc").unwrap();
    assert_eq!(fields.0.len(), 1);
    assert_eq!(fields.0.get("name"), Some(&SortOrder::Ascending));
}

#[test]
fn test_sort_fields_from_str_multiple() {
    let fields = SortFields::from_str("date_created:desc,name:asc,surname:asc").unwrap();
    assert_eq!(fields.0.len(), 3);

    assert_eq!(fields.0.get("date_created"), Some(&SortOrder::Descending));
    assert_eq!(fields.0.get("name"), Some(&SortOrder::Ascending));
    assert_eq!(fields.0.get("surname"), Some(&SortOrder::Ascending));
}

#[test]
fn test_sort_fields_from_str_with_whitespace() {
    let fields =
        SortFields::from_str("  date_created:desc  ,  name:asc  ,  surname:asc  ").unwrap();
    assert_eq!(fields.0.len(), 3);
    assert_eq!(fields.0.get("date_created"), Some(&SortOrder::Descending));
    assert_eq!(fields.0.get("name"), Some(&SortOrder::Ascending));
    assert_eq!(fields.0.get("surname"), Some(&SortOrder::Ascending));
}

#[test]
fn test_sort_fields_from_str_with_empty_fields() {
    let fields = SortFields::from_str("name:asc,,surname:asc").unwrap();
    assert_eq!(fields.0.len(), 2);
    assert_eq!(fields.0.get("name"), Some(&SortOrder::Ascending));
    assert_eq!(fields.0.get("surname"), Some(&SortOrder::Ascending));
}

#[test]
fn test_sort_fields_from_str_invalid() {
    // Invalid field format
    assert!(SortFields::from_str("name").is_err());
    assert!(SortFields::from_str("name:invalid").is_err());
    assert!(SortFields::from_str("name:asc,surname").is_err());
}

// ============================================================================
// SIMILARITY TESTS
// ============================================================================

#[test]
fn test_similarity_variants() {
    assert_eq!(Similarity::Equals, Similarity::Equals);
    assert_eq!(Similarity::Contains, Similarity::Contains);
    assert_eq!(Similarity::StartsWith, Similarity::StartsWith);
    assert_eq!(Similarity::EndsWith, Similarity::EndsWith);
    assert_eq!(Similarity::Between, Similarity::Between);
    assert_eq!(Similarity::Lesser, Similarity::Lesser);
    assert_eq!(Similarity::LesserOrEqual, Similarity::LesserOrEqual);
    assert_eq!(Similarity::Greater, Similarity::Greater);
    assert_eq!(Similarity::GreaterOrEqual, Similarity::GreaterOrEqual);
}

#[test]
fn test_similarity_constants() {
    assert_eq!(Similarity::EQUALS, "equals");
    assert_eq!(Similarity::CONTAINS, "contains");
    assert_eq!(Similarity::STARTS_WITH, "starts-with");
    assert_eq!(Similarity::ENDS_WITH, "ends-with");
    assert_eq!(Similarity::BETWEEN, "between");
    assert_eq!(Similarity::LESSER, "lesser");
    assert_eq!(Similarity::LESSER_OR_EQUAL, "lesser-or-equal");
    assert_eq!(Similarity::GREATER, "greater");
    assert_eq!(Similarity::GREATER_OR_EQUAL, "greater-or-equal");
}

#[test]
fn test_similarity_default() {
    assert_eq!(Similarity::default(), Similarity::Equals);
}

#[test]
fn test_similarity_from_str_valid() {
    assert_eq!(Similarity::from_str("equals").unwrap(), Similarity::Equals);
    assert_eq!(
        Similarity::from_str("contains").unwrap(),
        Similarity::Contains
    );
    assert_eq!(
        Similarity::from_str("starts-with").unwrap(),
        Similarity::StartsWith
    );
    assert_eq!(
        Similarity::from_str("ends-with").unwrap(),
        Similarity::EndsWith
    );
    assert_eq!(
        Similarity::from_str("between").unwrap(),
        Similarity::Between
    );
    assert_eq!(
        Similarity::from_str("lesser").unwrap(),
        Similarity::Lesser
    );
    assert_eq!(
        Similarity::from_str("lesser-or-equal").unwrap(),
        Similarity::LesserOrEqual
    );
    assert_eq!(
        Similarity::from_str("greater").unwrap(),
        Similarity::Greater
    );
    assert_eq!(
        Similarity::from_str("greater-or-equal").unwrap(),
        Similarity::GreaterOrEqual
    );
}

#[test]
fn test_similarity_from_str_invalid() {
    assert!(Similarity::from_str("invalid").is_err());
    assert!(Similarity::from_str("").is_err());
    assert!(Similarity::from_str("EQUALS").is_err());
    assert!(Similarity::from_str("starts_with").is_err());
    assert!(Similarity::from_str("ends_with").is_err());
    assert!(Similarity::from_str("BETWEEN").is_err());
    assert!(Similarity::from_str("lesser_than").is_err());
    assert!(Similarity::from_str("greater_than").is_err());
    assert!(Similarity::from_str("lesser_or_equal").is_err());
    assert!(Similarity::from_str("greater_or_equal").is_err());
}

#[test]
fn test_similarity_to_string() {
    assert_eq!(Similarity::Equals.to_string(), "equals");
    assert_eq!(Similarity::Contains.to_string(), "contains");
    assert_eq!(Similarity::StartsWith.to_string(), "starts-with");
    assert_eq!(Similarity::EndsWith.to_string(), "ends-with");
    assert_eq!(Similarity::Between.to_string(), "between");
    assert_eq!(Similarity::Lesser.to_string(), "lesser");
    assert_eq!(Similarity::LesserOrEqual.to_string(), "lesser-or-equal");
    assert_eq!(Similarity::Greater.to_string(), "greater");
    assert_eq!(Similarity::GreaterOrEqual.to_string(), "greater-or-equal");
}

// ============================================================================
// PARAMETERS BUILDER TESTS
// ============================================================================

#[test]
fn test_parameters_builder_equals() {
    let mut params = Parameters::new();
    let values = vec!["value1".to_string(), "value2".to_string()];
    params.equals("name".to_string(), values.clone());
    
    assert_eq!(params.0.len(), 1);
    assert!(params.0.contains_key("name"));
    let (similarity, param_values) = &params.0["name"];
    assert_eq!(*similarity, Similarity::Equals);
    assert_eq!(*param_values, values);
}

#[test]
fn test_parameters_builder_contains() {
    let mut params = Parameters::new();
    let values = vec!["value1".to_string(), "value2".to_string()];
    params.contains("name".to_string(), values.clone());
    
    assert_eq!(params.0.len(), 1);
    assert!(params.0.contains_key("name"));
    let (similarity, param_values) = &params.0["name"];
    assert_eq!(*similarity, Similarity::Contains);
    assert_eq!(*param_values, values);
}

#[test]
fn test_parameters_builder_starts_with() {
    let mut params = Parameters::new();
    let values = vec!["value1".to_string(), "value2".to_string()];
    params.starts_with("name".to_string(), values.clone());
    
    assert_eq!(params.0.len(), 1);
    assert!(params.0.contains_key("name"));
    let (similarity, param_values) = &params.0["name"];
    assert_eq!(*similarity, Similarity::StartsWith);
    assert_eq!(*param_values, values);
}

#[test]
fn test_parameters_builder_ends_with() {
    let mut params = Parameters::new();
    let values = vec!["value1".to_string(), "value2".to_string()];
    params.ends_with("name".to_string(), values.clone());
    
    assert_eq!(params.0.len(), 1);
    assert!(params.0.contains_key("name"));
    let (similarity, param_values) = &params.0["name"];
    assert_eq!(*similarity, Similarity::EndsWith);
    assert_eq!(*param_values, values);
}

#[test]
fn test_parameters_builder_between() {
    let mut params = Parameters::new();
    let values = vec!["20".to_string(), "30".to_string()];
    params.between("age".to_string(), values.clone());
    
    assert_eq!(params.0.len(), 1);
    assert!(params.0.contains_key("age"));
    let (similarity, param_values) = &params.0["age"];
    assert_eq!(*similarity, Similarity::Between);
    assert_eq!(*param_values, values);
}

#[test]
fn test_parameters_builder_lesser() {
    let mut params = Parameters::new();
    let values = vec!["100".to_string()];
    params.lesser("price".to_string(), values.clone());
    
    assert_eq!(params.0.len(), 1);
    assert!(params.0.contains_key("price"));
    let (similarity, param_values) = &params.0["price"];
    assert_eq!(*similarity, Similarity::Lesser);
    assert_eq!(*param_values, values);
}

#[test]
fn test_parameters_builder_lesser_or_equal() {
    let mut params = Parameters::new();
    let values = vec!["100".to_string()];
    params.lesser_or_equal("price".to_string(), values.clone());
    
    assert_eq!(params.0.len(), 1);
    assert!(params.0.contains_key("price"));
    let (similarity, param_values) = &params.0["price"];
    assert_eq!(*similarity, Similarity::LesserOrEqual);
    assert_eq!(*param_values, values);
}

#[test]
fn test_parameters_builder_greater() {
    let mut params = Parameters::new();
    let values = vec!["50".to_string()];
    params.greater("price".to_string(), values.clone());
    
    assert_eq!(params.0.len(), 1);
    assert!(params.0.contains_key("price"));
    let (similarity, param_values) = &params.0["price"];
    assert_eq!(*similarity, Similarity::Greater);
    assert_eq!(*param_values, values);
}

#[test]
fn test_parameters_builder_greater_or_equal() {
    let mut params = Parameters::new();
    let values = vec!["50".to_string()];
    params.greater_or_equal("price".to_string(), values.clone());
    
    assert_eq!(params.0.len(), 1);
    assert!(params.0.contains_key("price"));
    let (similarity, param_values) = &params.0["price"];
    assert_eq!(*similarity, Similarity::GreaterOrEqual);
    assert_eq!(*param_values, values);
}

#[test]
fn test_parameters_builder_fluent_api() {
    let mut params = Parameters::new();
    params.equals("name".to_string(), vec!["damian".to_string()])
          .contains("surname".to_string(), vec!["black".to_string()])
          .between("age".to_string(), vec!["20".to_string(), "30".to_string()]);
    
    assert_eq!(params.0.len(), 3);
    
    let (similarity, values) = &params.0["name"];
    assert_eq!(*similarity, Similarity::Equals);
    assert_eq!(*values, vec!["damian"]);
    
    let (similarity, values) = &params.0["surname"];
    assert_eq!(*similarity, Similarity::Contains);
    assert_eq!(*values, vec!["black"]);
    
    let (similarity, values) = &params.0["age"];
    assert_eq!(*similarity, Similarity::Between);
    assert_eq!(*values, vec!["20", "30"]);
}

#[test]
fn test_parameters_builder_overwrite() {
    let mut params = Parameters::new();
    params.equals("name".to_string(), vec!["damian".to_string()]);
    params.contains("name".to_string(), vec!["john".to_string()]); // Should overwrite
    
    assert_eq!(params.0.len(), 1);
    let (similarity, values) = &params.0["name"];
    assert_eq!(*similarity, Similarity::Contains);
    assert_eq!(*values, vec!["john"]);
}

#[test]
fn test_parameters_builder_empty_values() {
    let mut params = Parameters::new();
    params.equals("name".to_string(), vec![]);
    
    assert_eq!(params.0.len(), 1);
    let (similarity, values) = &params.0["name"];
    assert_eq!(*similarity, Similarity::Equals);
    assert_eq!(*values, vec![] as Vec<String>);
}

// ============================================================================
// PARAMETERS TESTS
// ============================================================================

#[test]
fn test_parameters_constants() {
    assert_eq!(Parameters::ORDER, "order");
    assert_eq!(Parameters::LIMIT, "limit");
    assert_eq!(Parameters::OFFSET, "offset");
    assert_eq!(Parameters::DEFAULT_LIMIT, 50);
    assert_eq!(Parameters::DEFAULT_OFFSET, 0);

    assert!(Parameters::EXCLUDE.contains(&"order"));
    assert!(Parameters::EXCLUDE.contains(&"limit"));
    assert!(Parameters::EXCLUDE.contains(&"offset"));
}

#[test]
fn test_parameters_new() {
    let params = Parameters::new();
    assert_eq!(params.0.len(), 0);
}

#[test]
fn test_parameters_default() {
    let params = Parameters::default();
    assert_eq!(params.0.len(), 0);
}

#[test]
fn test_parameters_from_str_empty() {
    let params = Parameters::from_str("").unwrap();
    assert_eq!(params.0.len(), 0);

    let params = Parameters::from_str("   ").unwrap();
    assert_eq!(params.0.len(), 0);
}

#[test]
fn test_parameters_from_str_single() {
    let params = Parameters::from_str("name=contains:damian").unwrap();
    assert_eq!(params.0.len(), 1);
    assert!(params.0.contains_key("name"));
    let (similarity, values) = &params.0["name"];
    assert_eq!(*similarity, Similarity::Contains);
    assert_eq!(*values, vec!["damian"]);
}

#[test]
fn test_parameters_from_str_multiple() {
    let params =
        Parameters::from_str("name=contains:damian&surname=equals:black,steel,wood").unwrap();
    assert_eq!(params.0.len(), 2);

    assert!(params.0.contains_key("name"));
    let (similarity, values) = &params.0["name"];
    assert_eq!(*similarity, Similarity::Contains);
    assert_eq!(*values, vec!["damian"]);

    assert!(params.0.contains_key("surname"));
    let (similarity, values) = &params.0["surname"];
    assert_eq!(*similarity, Similarity::Equals);
    assert_eq!(*values, vec!["black", "steel", "wood"]);
}

#[test]
fn test_parameters_from_str_with_whitespace() {
    let params =
        Parameters::from_str("  name  =  contains:damian  &  surname  =  equals:black  ").unwrap();
    assert_eq!(params.0.len(), 2);
    assert!(params.0.contains_key("name"));
    assert!(params.0.contains_key("surname"));
}

#[test]
fn test_parameters_from_str_excludes_special_params() {
    let params =
        Parameters::from_str("name=contains:damian&order=date_created:desc&limit=40&offset=0")
            .unwrap();
    assert_eq!(params.0.len(), 1);
    assert!(params.0.contains_key("name"));
    assert!(!params.0.contains_key("order"));
    assert!(!params.0.contains_key("limit"));
    assert!(!params.0.contains_key("offset"));
}

#[test]
fn test_parameters_from_str_empty_params() {
    let params = Parameters::from_str("name=contains:damian&&surname=equals:black&").unwrap();
    assert_eq!(params.0.len(), 2);
    assert!(params.0.contains_key("name"));
    assert!(params.0.contains_key("surname"));
}

#[test]
fn test_parameters_from_str_invalid_key() {
    let params = Parameters::from_str("=contains:damian").unwrap();
    assert_eq!(params.0.len(), 0);
}

#[test]
fn test_parameters_from_str_invalid() {
    // Invalid parameter format
    assert!(Parameters::from_str("name").is_err());
    assert!(Parameters::from_str("name=invalid:damian").is_err());
}

#[test]
fn test_parameters_from_str_with_numeric_comparisons() {
    let params = Parameters::from_str("age=between:20,30&price=greater:100&score=lesser-or-equal:85").unwrap();
    assert_eq!(params.0.len(), 3);

    assert!(params.0.contains_key("age"));
    let (similarity, values) = &params.0["age"];
    assert_eq!(*similarity, Similarity::Between);
    assert_eq!(*values, vec!["20", "30"]);

    assert!(params.0.contains_key("price"));
    let (similarity, values) = &params.0["price"];
    assert_eq!(*similarity, Similarity::Greater);
    assert_eq!(*values, vec!["100"]);

    assert!(params.0.contains_key("score"));
    let (similarity, values) = &params.0["score"];
    assert_eq!(*similarity, Similarity::LesserOrEqual);
    assert_eq!(*values, vec!["85"]);
}

#[test]
fn test_parameters_from_str_mixed_similarity_types() {
    let params = Parameters::from_str("name=contains:damian&age=between:25,35&price=greater-or-equal:50&status=equals:active").unwrap();
    assert_eq!(params.0.len(), 4);

    assert!(params.0.contains_key("name"));
    let (similarity, values) = &params.0["name"];
    assert_eq!(*similarity, Similarity::Contains);
    assert_eq!(*values, vec!["damian"]);

    assert!(params.0.contains_key("age"));
    let (similarity, values) = &params.0["age"];
    assert_eq!(*similarity, Similarity::Between);
    assert_eq!(*values, vec!["25", "35"]);

    assert!(params.0.contains_key("price"));
    let (similarity, values) = &params.0["price"];
    assert_eq!(*similarity, Similarity::GreaterOrEqual);
    assert_eq!(*values, vec!["50"]);

    assert!(params.0.contains_key("status"));
    let (similarity, values) = &params.0["status"];
    assert_eq!(*similarity, Similarity::Equals);
    assert_eq!(*values, vec!["active"]);
}

#[test]
fn test_parameters_from_str_numeric_with_whitespace() {
    let params = Parameters::from_str("  age  =  between:20,30  &  price  =  lesser:100  ").unwrap();
    assert_eq!(params.0.len(), 2);
    assert!(params.0.contains_key("age"));
    assert!(params.0.contains_key("price"));
}

// ============================================================================
// QUERY TESTS
// ============================================================================

#[test]
fn test_query_new() {
    let query = Query::new();
    assert_eq!(query.parameters.0.len(), 0);
    assert_eq!(query.sort_fields.0.len(), 0);
    assert_eq!(query.limit, Parameters::DEFAULT_LIMIT);
    assert_eq!(query.offset, Parameters::DEFAULT_OFFSET);
}

#[test]
fn test_query_init() {
    let params = Parameters::new();
    let sort_fields = SortFields::new();
    let query = Query::init(params, sort_fields, 100, 10);
    assert_eq!(query.parameters.0.len(), 0);
    assert_eq!(query.sort_fields.0.len(), 0);
    assert_eq!(query.limit, 100);
    assert_eq!(query.offset, 10);
}

#[test]
fn test_query_to_http_empty() {
    let query = Query::new();
    let http = query.to_http();
    assert_eq!(http, "limit=50&offset=0");
}

#[test]
fn test_query_to_http_with_params() {
    let mut query = Query::new();
    query.parameters.0.insert("name".to_string(), (Similarity::Contains, vec!["damian".to_string()]));

    let http = query.to_http();
    assert!(http.contains("name=contains:damian"));
    assert!(http.contains("limit=50"));
    assert!(http.contains("offset=0"));
}

#[test]
fn test_query_to_http_with_sort() {
    let mut query = Query::new();
    query.sort_fields.desc("date_created".to_string());

    let http = query.to_http();
    assert!(http.contains("date_created:desc"));
    assert!(http.contains("limit=50"));
    assert!(http.contains("offset=0"));
}

#[test]
fn test_query_to_http_with_params_and_sort() {
    let mut query = Query::new();
    query.parameters.0.insert("name".to_string(), (Similarity::Contains, vec!["damian".to_string()]));

    query.sort_fields.desc("date_created".to_string());

    let http = query.to_http();
    assert!(http.contains("name=contains:damian"));
    assert!(http.contains("date_created:desc"));
    assert!(http.contains("limit=50"));
    assert!(http.contains("offset=0"));
}

#[test]
fn test_query_to_http_sort_fields_empty_values() {
    let mut query = Query::new();
    query.parameters.0.insert("name".to_string(), (Similarity::Contains, vec![]));

    let http = query.to_http();
    assert!(!http.contains("name="));
    assert_eq!(http, "limit=50&offset=0");
}

#[test]
fn test_query_to_http_empty_sort_fields() {
    let mut query = Query::new();
    query.sort_fields.asc("".to_string());

    let http = query.to_http();
    assert!(!http.contains(":asc"));
    assert_eq!(http, "limit=50&offset=0");
}

#[test]
fn test_query_from_http_empty() {
    let query = Query::from_http("".to_string()).unwrap();
    assert_eq!(query.parameters.0.len(), 0);
    assert_eq!(query.sort_fields.0.len(), 0);
    assert_eq!(query.limit, Parameters::DEFAULT_LIMIT);
    assert_eq!(query.offset, Parameters::DEFAULT_OFFSET);
}

#[test]
fn test_query_from_http_with_question_mark() {
    let query = Query::from_http("?name=contains:damian".to_string()).unwrap();
    assert_eq!(query.parameters.0.len(), 1);
    assert!(query.parameters.0.contains_key("name"));
}

#[test]
fn test_query_from_http_with_params() {
    let query = Query::from_http("name=contains:damian&surname=equals:black".to_string()).unwrap();
    assert_eq!(query.parameters.0.len(), 2);
    assert!(query.parameters.0.contains_key("name"));
    assert!(query.parameters.0.contains_key("surname"));
}

#[test]
fn test_query_from_http_with_order() {
    let query = Query::from_http("order=date_created:desc,name:asc".to_string()).unwrap();
    assert_eq!(query.sort_fields.0.len(), 2);
    assert_eq!(query.sort_fields.0.get("date_created"), Some(&SortOrder::Descending));
    assert_eq!(query.sort_fields.0.get("name"), Some(&SortOrder::Ascending));
}

#[test]
fn test_query_from_http_with_limit() {
    let query = Query::from_http("limit=100".to_string()).unwrap();
    assert_eq!(query.limit, 100);
}

#[test]
fn test_query_from_http_with_offset() {
    let query = Query::from_http("offset=20".to_string()).unwrap();
    assert_eq!(query.offset, 20);
}

#[test]
fn test_query_from_http_with_invalid_limit() {
    let query = Query::from_http("limit=invalid".to_string()).unwrap();
    assert_eq!(query.limit, Parameters::DEFAULT_LIMIT);
}

#[test]
fn test_query_from_http_with_invalid_offset() {
    let query = Query::from_http("offset=invalid".to_string()).unwrap();
    assert_eq!(query.offset, Parameters::DEFAULT_OFFSET);
}

#[test]
fn test_query_from_http_complete() {
    let query = Query::from_http("name=contains:damian&surname=equals:black,steel,wood&order=date_created:desc&limit=40&offset=0".to_string()).unwrap();

    assert_eq!(query.parameters.0.len(), 2);
    assert!(query.parameters.0.contains_key("name"));
    assert!(query.parameters.0.contains_key("surname"));

    assert_eq!(query.sort_fields.0.len(), 1);
    assert_eq!(query.sort_fields.0.get("date_created"), Some(&SortOrder::Descending));

    assert_eq!(query.limit, 40);
    assert_eq!(query.offset, 0);
}

#[test]
fn test_query_from_http_with_whitespace() {
    let query = Query::from_http(
        "  name  =  contains:damian  &  order  =  date_created:desc  ".to_string(),
    )
    .unwrap();
    assert_eq!(query.parameters.0.len(), 1);
    assert!(query.parameters.0.contains_key("name"));
    assert_eq!(query.sort_fields.0.len(), 1);
    assert_eq!(query.sort_fields.0.get("date_created"), Some(&SortOrder::Descending));
}

#[test]
fn test_query_from_http_empty_values() {
    let query = Query::from_http("name=&order=&limit=&offset=".to_string()).unwrap();
    assert_eq!(query.parameters.0.len(), 0);
    assert_eq!(query.sort_fields.0.len(), 0);
    assert_eq!(query.limit, Parameters::DEFAULT_LIMIT);
    assert_eq!(query.offset, Parameters::DEFAULT_OFFSET);
}

#[test]
fn test_query_from_http_invalid() {
    // Missing value
    assert!(Query::from_http("name".to_string()).is_err());

    // Invalid parameter format
    assert!(Query::from_http("name=invalid:damian".to_string()).is_err());

    // Invalid order format
    assert!(Query::from_http("order=invalid".to_string()).is_err());
}

#[test]
fn test_parameters_keep() {
    let mut params = Parameters::new();
    params.0.insert("name".to_string(), (Similarity::Contains, vec!["value1".to_string()]));
    params.0.insert("surname".to_string(), (Similarity::Equals, vec!["value2".to_string()]));
    params.0.insert("email".to_string(), (Similarity::StartsWith, vec!["value3".to_string()]));

    let filtered = params.keep(vec!["name".to_string(), "email".to_string()]);

    assert_eq!(filtered.0.len(), 2);
    assert!(filtered.0.contains_key("name"));
    assert!(!filtered.0.contains_key("surname"));
    assert!(filtered.0.contains_key("email"));
}

#[test]
fn test_parameters_keep_nonexistent_keys() {
    let mut params = Parameters::new();
    params.0.insert("name".to_string(), (Similarity::Contains, vec!["value1".to_string()]));

    let filtered = params.keep(vec!["nonexistent".to_string()]);
    assert_eq!(filtered.0.len(), 0);
}

#[test]
fn test_parameters_keep_empty_keys() {
    let mut params = Parameters::new();
    params.0.insert("name".to_string(), (Similarity::Contains, vec!["value1".to_string()]));

    let filtered = params.keep(vec![]);
    assert_eq!(filtered.0.len(), 0);
}

#[test]
fn test_parameters_remove() {
    let mut params = Parameters::new();
    params.0.insert("name".to_string(), (Similarity::Contains, vec!["value1".to_string()]));
    params.0.insert("surname".to_string(), (Similarity::Equals, vec!["value2".to_string()]));
    params.0.insert("email".to_string(), (Similarity::StartsWith, vec!["value3".to_string()]));

    let filtered = params.remove(vec!["name".to_string(), "email".to_string()]);

    assert_eq!(filtered.0.len(), 1);
    assert!(!filtered.0.contains_key("name"));
    assert!(filtered.0.contains_key("surname"));
    assert!(!filtered.0.contains_key("email"));
}

#[test]
fn test_parameters_remove_nonexistent_keys() {
    let mut params = Parameters::new();
    params.0.insert("name".to_string(), (Similarity::Contains, vec!["value1".to_string()]));

    let filtered = params.remove(vec!["nonexistent".to_string()]);
    assert_eq!(filtered.0.len(), 1);
    assert!(filtered.0.contains_key("name"));
}

#[test]
fn test_parameters_remove_empty_keys() {
    let mut params = Parameters::new();
    params.0.insert("name".to_string(), (Similarity::Contains, vec!["value1".to_string()]));

    let filtered = params.remove(vec![]);
    assert_eq!(filtered.0.len(), 1);
    assert!(filtered.0.contains_key("name"));
}

#[test]
fn test_query_from_http_with_numeric_comparisons() {
    let query = Query::from_http("age=between:20,30&price=greater:100&score=lesser-or-equal:85&order=date_created:desc&limit=25&offset=10".to_string()).unwrap();
    
    assert_eq!(query.parameters.0.len(), 3);
    
    assert!(query.parameters.0.contains_key("age"));
    let (similarity, values) = &query.parameters.0["age"];
    assert_eq!(*similarity, Similarity::Between);
    assert_eq!(*values, vec!["20", "30"]);
    
    assert!(query.parameters.0.contains_key("price"));
    let (similarity, values) = &query.parameters.0["price"];
    assert_eq!(*similarity, Similarity::Greater);
    assert_eq!(*values, vec!["100"]);
    
    assert!(query.parameters.0.contains_key("score"));
    let (similarity, values) = &query.parameters.0["score"];
    assert_eq!(*similarity, Similarity::LesserOrEqual);
    assert_eq!(*values, vec!["85"]);
    
    assert_eq!(query.sort_fields.0.len(), 1);
    assert_eq!(query.sort_fields.0.get("date_created"), Some(&SortOrder::Descending));
    
    assert_eq!(query.limit, 25);
    assert_eq!(query.offset, 10);
}

#[test]
fn test_query_from_http_mixed_similarity_types() {
    let query = Query::from_http("name=contains:damian&age=between:25,35&price=greater-or-equal:50&status=equals:active&order=name:asc&limit=20".to_string()).unwrap();
    
    assert_eq!(query.parameters.0.len(), 4);
    
    assert!(query.parameters.0.contains_key("name"));
    let (similarity, values) = &query.parameters.0["name"];
    assert_eq!(*similarity, Similarity::Contains);
    assert_eq!(*values, vec!["damian"]);
    
    assert!(query.parameters.0.contains_key("age"));
    let (similarity, values) = &query.parameters.0["age"];
    assert_eq!(*similarity, Similarity::Between);
    assert_eq!(*values, vec!["25", "35"]);
    
    assert!(query.parameters.0.contains_key("price"));
    let (similarity, values) = &query.parameters.0["price"];
    assert_eq!(*similarity, Similarity::GreaterOrEqual);
    assert_eq!(*values, vec!["50"]);
    
    assert!(query.parameters.0.contains_key("status"));
    let (similarity, values) = &query.parameters.0["status"];
    assert_eq!(*similarity, Similarity::Equals);
    assert_eq!(*values, vec!["active"]);
    
    assert_eq!(query.sort_fields.0.len(), 1);
    assert_eq!(query.sort_fields.0.get("name"), Some(&SortOrder::Ascending));
    
    assert_eq!(query.limit, 20);
}

#[test]
fn test_query_to_http_with_numeric_comparisons() {
    let mut query = Query::new();
    
    query.parameters.0.insert("age".to_string(), (Similarity::Between, vec!["20".to_string(), "30".to_string()]));
    query.parameters.0.insert("price".to_string(), (Similarity::Greater, vec!["100".to_string()]));
    query.parameters.0.insert("score".to_string(), (Similarity::LesserOrEqual, vec!["85".to_string()]));
    
    query.sort_fields.desc("date_created".to_string());
    
    query.limit = 25;
    query.offset = 10;
    
    let http = query.to_http();
    
    assert!(http.contains("age=between:20,30"));
    assert!(http.contains("price=greater:100"));
    assert!(http.contains("score=lesser-or-equal:85"));
    assert!(http.contains("date_created:desc"));
    assert!(http.contains("limit=25"));
    assert!(http.contains("offset=10"));
}

#[test]
fn test_parameters_keep_with_numeric_comparisons() {
    let mut params = Parameters::new();
    
    params.0.insert("age".to_string(), (Similarity::Between, vec!["20".to_string(), "30".to_string()]));
    params.0.insert("price".to_string(), (Similarity::Greater, vec!["100".to_string()]));
    params.0.insert("name".to_string(), (Similarity::Contains, vec!["damian".to_string()]));
    
    let filtered = params.keep(vec!["age".to_string(), "name".to_string()]);
    
    assert_eq!(filtered.0.len(), 2);
    assert!(filtered.0.contains_key("age"));
    assert!(!filtered.0.contains_key("price"));
    assert!(filtered.0.contains_key("name"));
    
    let (similarity, values) = &filtered.0["age"];
    assert_eq!(*similarity, Similarity::Between);
    assert_eq!(*values, vec!["20", "30"]);
}

#[test]
fn test_parameters_remove_with_numeric_comparisons() {
    let mut params = Parameters::new();
    
    params.0.insert("age".to_string(), (Similarity::Between, vec!["20".to_string(), "30".to_string()]));
    params.0.insert("price".to_string(), (Similarity::Greater, vec!["100".to_string()]));
    params.0.insert("name".to_string(), (Similarity::Contains, vec!["damian".to_string()]));
    
    let filtered = params.remove(vec!["age".to_string(), "name".to_string()]);
    
    assert_eq!(filtered.0.len(), 1);
    assert!(!filtered.0.contains_key("age"));
    assert!(filtered.0.contains_key("price"));
    assert!(!filtered.0.contains_key("name"));
    
    let (similarity, values) = &filtered.0["price"];
    assert_eq!(*similarity, Similarity::Greater);
    assert_eq!(*values, vec!["100"]);
}

// ============================================================================
// ERROR HANDLING TESTS
// ============================================================================

#[test]
fn test_error_invalid_sort_order() {
    let error = SortOrder::from_str("invalid").unwrap_err();
    match error {
        Error::InvalidSortOrder(msg) => assert_eq!(msg, "invalid"),
        _ => panic!("Expected InvalidSortOrder error"),
    }
}

#[test]
fn test_error_invalid_sort_field() {
    let error = SortFields::from_str("invalid").unwrap_err();
    match error {
        Error::InvalidSortField(msg) => assert_eq!(msg, "invalid"),
        _ => panic!("Expected InvalidSortField error"),
    }
}

#[test]
fn test_error_invalid_similarity() {
    let error = Similarity::from_str("invalid").unwrap_err();
    match error {
        Error::InvalidSimilarity(msg) => assert_eq!(msg, "invalid"),
        _ => panic!("Expected InvalidSimilarity error"),
    }
}

#[test]
fn test_error_invalid_parameter() {
    let error = parse_parameter("invalid").unwrap_err();
    match error {
        Error::InvalidParameter(msg) => assert_eq!(msg, "invalid"),
        _ => panic!("Expected InvalidParameter error"),
    }
}

#[test]
fn test_error_invalid_search_parameters() {
    let error = Query::from_http("name".to_string()).unwrap_err();
    match error {
        Error::InvalidSearchParameters(msg) => assert_eq!(msg, "name"),
        _ => panic!("Expected InvalidSearchParameters error"),
    }
}

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

#[test]
fn test_roundtrip_simple_query() {
    let original = "name=contains:damian&limit=25&offset=10";
    let query = Query::from_http(original.to_string()).unwrap();
    let reconstructed = query.to_http();

    // The reconstructed query should contain the same information
    assert!(reconstructed.contains("name=contains:damian"));
    assert!(reconstructed.contains("limit=25"));
    assert!(reconstructed.contains("offset=10"));
}

#[test]
fn test_roundtrip_complex_query() {
    let original = "name=contains:damian&surname=equals:black,steel,wood&order=date_created:desc,name:asc&limit=40&offset=0";
    let query = Query::from_http(original.to_string()).unwrap();
    let reconstructed = query.to_http();

    // Verify all components are preserved
    assert!(reconstructed.contains("name=contains:damian"));
    assert!(reconstructed.contains("surname=equals:black,steel,wood"));
    assert!(reconstructed.contains("date_created:desc"));
    assert!(reconstructed.contains("name:asc"));
    assert!(reconstructed.contains("limit=40"));
    assert!(reconstructed.contains("offset=0"));
}

#[test]
fn test_complex_sort_fields_parsing() {
    let sort_str = "date_created:desc,name:asc,surname:asc,email:desc";
    let fields = SortFields::from_str(sort_str).unwrap();

    assert_eq!(fields.0.len(), 4);
    assert_eq!(fields.0.get("date_created"), Some(&SortOrder::Descending));
    assert_eq!(fields.0.get("name"), Some(&SortOrder::Ascending));
    assert_eq!(fields.0.get("surname"), Some(&SortOrder::Ascending));
    assert_eq!(fields.0.get("email"), Some(&SortOrder::Descending));
}

#[test]
fn test_complex_parameters_parsing() {
    let param_str = "name=contains:damian&surname=equals:black,steel,wood&email=starts-with:test&age=ends-with:25";
    let params = Parameters::from_str(param_str).unwrap();

    assert_eq!(params.0.len(), 4);

    let (similarity, values) = &params.0["name"];
    assert_eq!(*similarity, Similarity::Contains);
    assert_eq!(*values, vec!["damian"]);

    let (similarity, values) = &params.0["surname"];
    assert_eq!(*similarity, Similarity::Equals);
    assert_eq!(*values, vec!["black", "steel", "wood"]);

    let (similarity, values) = &params.0["email"];
    assert_eq!(*similarity, Similarity::StartsWith);
    assert_eq!(*values, vec!["test"]);

    let (similarity, values) = &params.0["age"];
    assert_eq!(*similarity, Similarity::EndsWith);
    assert_eq!(*values, vec!["25"]);
}

#[test]
fn test_edge_case_whitespace_handling() {
    // Test various whitespace scenarios
    let query_str = "  name  =  contains  :  damian  &  order  =  date_created  :  desc  ";
    let query = Query::from_http(query_str.to_string()).unwrap();

    assert_eq!(query.parameters.0.len(), 1);
    assert!(query.parameters.0.contains_key("name"));

    let (similarity, values) = &query.parameters.0["name"];
    assert_eq!(*similarity, Similarity::Contains);
    assert_eq!(*values, vec!["damian"]);

    assert_eq!(query.sort_fields.0.len(), 1);
    assert_eq!(query.sort_fields.0.get("date_created"), Some(&SortOrder::Descending));
}

#[test]
fn test_edge_case_empty_and_whitespace_values() {
    let query_str = "name=contains:&surname=equals:,,&order=:desc&limit=&offset=";
    let query = Query::from_http(query_str.to_string()).unwrap();

    // Empty values should be filtered out
    assert_eq!(query.parameters.0.len(), 0);
    assert_eq!(query.sort_fields.0.len(), 0);
    assert_eq!(query.limit, Parameters::DEFAULT_LIMIT);
    assert_eq!(query.offset, Parameters::DEFAULT_OFFSET);
}

#[test]
fn test_edge_case_special_characters_in_values() {
    let query_str = "name=contains:damian%20test&surname=equals:black,steel,wood";
    let query = Query::from_http(query_str.to_string()).unwrap();

    assert_eq!(query.parameters.0.len(), 2);

    let (_, values) = &query.parameters.0["name"];
    assert_eq!(*values, vec!["damian test"]);

    let (_, values) = &query.parameters.0["surname"];
    assert_eq!(*values, vec!["black", "steel", "wood"]);
}

#[test]
fn test_edge_case_very_long_query() {
    let mut query_str = String::new();
    for i in 0..100 {
        query_str.push_str(&format!("param{}={}:value{}&", i, "contains", i));
    }
    query_str.push_str("order=name:asc&limit=50&offset=0");

    let query = Query::from_http(query_str).unwrap();
    assert_eq!(query.parameters.0.len(), 100);
    assert_eq!(query.sort_fields.0.len(), 1);
    assert_eq!(query.limit, 50);
    assert_eq!(query.offset, 0);
}

#[test]
fn test_edge_case_unicode_characters() {
    let query_str = "name=contains:damian_测试&surname=equals:black,steel,wood";
    let query = Query::from_http(query_str.to_string()).unwrap();

    assert_eq!(query.parameters.0.len(), 2);

    let (_, values) = &query.parameters.0["name"];
    assert_eq!(*values, vec!["damian_测试"]);
}

#[test]
fn test_edge_case_case_sensitivity() {
    // Test that similarity and sort order are case sensitive
    assert!(Similarity::from_str("EQUALS").is_err());
    assert!(Similarity::from_str("Contains").is_err());

    assert!(SortOrder::from_str("ASC").is_err());
    assert!(SortOrder::from_str("DESC").is_err());

    // But parameter keys should be preserved as-is
    let query_str = "Name=contains:damian&NAME=equals:test";
    let query = Query::from_http(query_str.to_string()).unwrap();
    assert_eq!(query.parameters.0.len(), 2);
    assert!(query.parameters.0.contains_key("Name"));
    assert!(query.parameters.0.contains_key("NAME"));
}

#[test]
fn test_roundtrip_numeric_comparisons() {
    let original = "age=between:20,30&price=greater:100&score=lesser-or-equal:85&order=date_created:desc&limit=25&offset=10";
    let query = Query::from_http(original.to_string()).unwrap();
    let reconstructed = query.to_http();

    // Verify all numeric comparison components are preserved
    assert!(reconstructed.contains("age=between:20,30"));
    assert!(reconstructed.contains("price=greater:100"));
    assert!(reconstructed.contains("score=lesser-or-equal:85"));
    assert!(reconstructed.contains("date_created:desc"));
    assert!(reconstructed.contains("limit=25"));
    assert!(reconstructed.contains("offset=10"));
}

#[test]
fn test_roundtrip_mixed_similarity_types() {
    let original = "name=contains:damian&age=between:25,35&price=greater-or-equal:50&status=equals:active&order=name:asc&limit=20";
    let query = Query::from_http(original.to_string()).unwrap();
    let reconstructed = query.to_http();

    // Verify all mixed similarity types are preserved
    assert!(reconstructed.contains("name=contains:damian"));
    assert!(reconstructed.contains("age=between:25,35"));
    assert!(reconstructed.contains("price=greater-or-equal:50"));
    assert!(reconstructed.contains("status=equals:active"));
    assert!(reconstructed.contains("name:asc"));
    assert!(reconstructed.contains("limit=20"));
}

#[test]
fn test_complex_numeric_parameters_parsing() {
    let param_str = "age=between:20,30&price=greater:100&score=lesser-or-equal:85&rating=greater-or-equal:4.5&discount=lesser:10";
    let params = Parameters::from_str(param_str).unwrap();

    assert_eq!(params.0.len(), 5);

    let (similarity, values) = &params.0["age"];
    assert_eq!(*similarity, Similarity::Between);
    assert_eq!(*values, vec!["20", "30"]);

    let (similarity, values) = &params.0["price"];
    assert_eq!(*similarity, Similarity::Greater);
    assert_eq!(*values, vec!["100"]);

    let (similarity, values) = &params.0["score"];
    assert_eq!(*similarity, Similarity::LesserOrEqual);
    assert_eq!(*values, vec!["85"]);

    let (similarity, values) = &params.0["rating"];
    assert_eq!(*similarity, Similarity::GreaterOrEqual);
    assert_eq!(*values, vec!["4.5"]);

    let (similarity, values) = &params.0["discount"];
    assert_eq!(*similarity, Similarity::Lesser);
    assert_eq!(*values, vec!["10"]);
}

#[test]
fn test_edge_case_numeric_comparisons_with_whitespace() {
    let query_str = "  age  =  between:20,30  &  price  =  greater:100  &  score  =  lesser-or-equal:85  ";
    let query = Query::from_http(query_str.to_string()).unwrap();

    assert_eq!(query.parameters.0.len(), 3);

    let (similarity, values) = &query.parameters.0["age"];
    assert_eq!(*similarity, Similarity::Between);
    assert_eq!(*values, vec!["20", "30"]);

    let (similarity, values) = &query.parameters.0["price"];
    assert_eq!(*similarity, Similarity::Greater);
    assert_eq!(*values, vec!["100"]);

    let (similarity, values) = &query.parameters.0["score"];
    assert_eq!(*similarity, Similarity::LesserOrEqual);
    assert_eq!(*values, vec!["85"]);
}

#[test]
fn test_edge_case_numeric_comparisons_empty_values() {
    let query_str = "age=between:&price=greater:&score=lesser-or-equal:";
    let query = Query::from_http(query_str.to_string()).unwrap();

    // Empty values should be filtered out
    assert_eq!(query.parameters.0.len(), 0);
}

#[test]
fn test_edge_case_numeric_comparisons_mixed_empty_values() {
    let query_str = "age=between:20,30&price=greater:&score=lesser-or-equal:85&name=contains:";
    let query = Query::from_http(query_str.to_string()).unwrap();

    // Only parameters with values should be included
    assert_eq!(query.parameters.0.len(), 2);
    assert!(query.parameters.0.contains_key("age"));
    assert!(query.parameters.0.contains_key("score"));
    assert!(!query.parameters.0.contains_key("price"));
    assert!(!query.parameters.0.contains_key("name"));
}

#[test]
fn test_edge_case_numeric_comparisons_special_characters() {
    let query_str = "price=between:100.50,200.75&discount=greater:10%&score=lesser-or-equal:85.5";
    let query = Query::from_http(query_str.to_string()).unwrap();

    assert_eq!(query.parameters.0.len(), 3);

    let (similarity, values) = &query.parameters.0["price"];
    assert_eq!(*similarity, Similarity::Between);
    assert_eq!(*values, vec!["100.50", "200.75"]);

    let (similarity, values) = &query.parameters.0["discount"];
    assert_eq!(*similarity, Similarity::Greater);
    assert_eq!(*values, vec!["10%"]);

    let (similarity, values) = &query.parameters.0["score"];
    assert_eq!(*similarity, Similarity::LesserOrEqual);
    assert_eq!(*values, vec!["85.5"]);
}

// ============================================================================
// SQL TESTS
// ============================================================================

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_empty() {
    let query = Query::new();
    let sql = query.to_sql();
    assert_eq!(sql, "LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_equals() {
    let mut query = Query::new();
    query.parameters.0.insert("name".to_string(), (Similarity::Equals, vec!["damian".to_string()]));
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name = ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_equals_multiple_values() {
    let mut query = Query::new();
    query.parameters.0.insert("name".to_string(), (Similarity::Equals, vec!["damian".to_string(), "john".to_string()]));
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name IN (?, ?) LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_equals_null() {
    let mut query = Query::new();
    query.parameters.0.insert("name".to_string(), (Similarity::Equals, vec!["null".to_string()]));
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name IS ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_contains() {
    let mut query = Query::new();
    query.parameters.0.insert("name".to_string(), (Similarity::Contains, vec!["damian".to_string()]));
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name LIKE ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_contains_multiple_values() {
    let mut query = Query::new();
    query.parameters.0.insert("name".to_string(), (Similarity::Contains, vec!["damian".to_string(), "john".to_string()]));
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE (name LIKE ? OR name LIKE ?) LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_starts_with() {
    let mut query = Query::new();
    query.parameters.0.insert("name".to_string(), (Similarity::StartsWith, vec!["damian".to_string()]));
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name LIKE ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_ends_with() {
    let mut query = Query::new();
    query.parameters.0.insert("name".to_string(), (Similarity::EndsWith, vec!["damian".to_string()]));
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name LIKE ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_between() {
    let mut query = Query::new();
    query.parameters.0.insert("age".to_string(), (Similarity::Between, vec!["20".to_string(), "30".to_string()]));
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE age BETWEEN ? AND ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_lesser() {
    let mut query = Query::new();
    query.parameters.0.insert("price".to_string(), (Similarity::Lesser, vec!["100".to_string()]));
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE price < ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_lesser_or_equal() {
    let mut query = Query::new();
    query.parameters.0.insert("price".to_string(), (Similarity::LesserOrEqual, vec!["100".to_string()]));
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE price <= ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_greater() {
    let mut query = Query::new();
    query.parameters.0.insert("price".to_string(), (Similarity::Greater, vec!["50".to_string()]));
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE price > ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_greater_or_equal() {
    let mut query = Query::new();
    query.parameters.0.insert("price".to_string(), (Similarity::GreaterOrEqual, vec!["50".to_string()]));
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE price >= ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_order_by() {
    let mut query = Query::new();
    query.sort_fields.desc("date_created".to_string());
    
    let sql = query.to_sql();
    assert_eq!(sql, "ORDER BY date_created DESC LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_multiple_order_by() {
    let mut query = Query::new();
    query.sort_fields.desc("date_created".to_string());
    query.sort_fields.asc("name".to_string());
    
    let sql = query.to_sql();
    assert_eq!(sql, "ORDER BY date_created DESC, name ASC LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_complex() {
    let mut query = Query::new();
    
    // Add multiple parameters
    query.parameters.0.insert("name".to_string(), (Similarity::Contains, vec!["damian".to_string()]));
    query.parameters.0.insert("age".to_string(), (Similarity::Between, vec!["20".to_string(), "30".to_string()]));
    query.parameters.0.insert("price".to_string(), (Similarity::Greater, vec!["100".to_string()]));
    
    // Add sorting
    query.sort_fields.desc("date_created".to_string());
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name LIKE ? AND age BETWEEN ? AND ? AND price > ? ORDER BY date_created DESC LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_empty_parameters() {
    let mut query = Query::new();
    query.parameters.0.insert("name".to_string(), (Similarity::Contains, vec![]));
    
    let sql = query.to_sql();
    assert_eq!(sql, "LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_empty_sort_fields() {
    let mut query = Query::new();
    query.sort_fields.asc("".to_string());
    
    let sql = query.to_sql();
    assert_eq!(sql, "LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_numeric_comparisons_multiple_values() {
    let mut query = Query::new();
    query.parameters.0.insert("price".to_string(), (Similarity::Greater, vec!["50".to_string(), "100".to_string()]));
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE (price > ? OR price > ?) LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_between_invalid_values() {
    let mut query = Query::new();
    query.parameters.0.insert("age".to_string(), (Similarity::Between, vec!["20".to_string()]));
    
    let sql = query.to_sql();
    assert_eq!(sql, "LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_between_multiple_pairs() {
    let mut query = Query::new();
    query.parameters.0.insert("age".to_string(), (Similarity::Between, vec!["10".to_string(), "20".to_string(), "30".to_string(), "40".to_string()]));
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE (age BETWEEN ? AND ? OR age BETWEEN ? AND ?) LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_between_odd_values_ignored() {
    let mut query = Query::new();
    query.parameters.0.insert("age".to_string(), (Similarity::Between, vec!["10".to_string(), "20".to_string(), "30".to_string(), "40".to_string(), "50".to_string()]));
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE (age BETWEEN ? AND ? OR age BETWEEN ? AND ?) LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_between_three_pairs() {
    let mut query = Query::new();
    query.parameters.0.insert("age".to_string(), (Similarity::Between, vec!["10".to_string(), "20".to_string(), "30".to_string(), "40".to_string(), "50".to_string(), "60".to_string()]));
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE (age BETWEEN ? AND ? OR age BETWEEN ? AND ? OR age BETWEEN ? AND ?) LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_between_single_pair() {
    let mut query = Query::new();
    query.parameters.0.insert("age".to_string(), (Similarity::Between, vec!["20".to_string(), "30".to_string()]));
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE age BETWEEN ? AND ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_between_empty_values() {
    let mut query = Query::new();
    query.parameters.0.insert("age".to_string(), (Similarity::Between, vec![]));
    
    let sql = query.to_sql();
    assert_eq!(sql, "LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_between_complex_with_other_conditions() {
    let mut query = Query::new();
    
    // Add between with multiple pairs
    query.parameters.0.insert("age".to_string(), (Similarity::Between, vec!["10".to_string(), "20".to_string(), "30".to_string(), "40".to_string(), "50".to_string()]));
    
    // Add other condition
    query.parameters.0.insert("name".to_string(), (Similarity::Contains, vec!["damian".to_string()]));
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE (age BETWEEN ? AND ? OR age BETWEEN ? AND ?) AND name LIKE ? LIMIT ? OFFSET ?");
}

// ============================================================================
// NORMAL QUERY PARAMETER TESTS
// ============================================================================

#[test]
fn test_query_from_http_normal_parameters() {
    let query = Query::from_http("name=ben&age=20".to_string()).unwrap();
    
    assert_eq!(query.parameters.0.len(), 2);
    
    assert!(query.parameters.0.contains_key("name"));
    let (similarity, values) = &query.parameters.0["name"];
    assert_eq!(*similarity, Similarity::Equals);
    assert_eq!(*values, vec!["ben"]);
    
    assert!(query.parameters.0.contains_key("age"));
    let (similarity, values) = &query.parameters.0["age"];
    assert_eq!(*similarity, Similarity::Equals);
    assert_eq!(*values, vec!["20"]);
}

#[test]
fn test_query_from_http_repeated_parameters() {
    let query = Query::from_http("name=ben&name=john&name=alice".to_string()).unwrap();
    
    assert_eq!(query.parameters.0.len(), 1);
    
    assert!(query.parameters.0.contains_key("name"));
    let (similarity, values) = &query.parameters.0["name"];
    assert_eq!(*similarity, Similarity::Equals);
    assert_eq!(*values, vec!["ben", "john", "alice"]);
}

#[test]
fn test_query_from_http_mixed_normal_and_similarity() {
    let query = Query::from_http("name=ben&name=john&age=contains:20&status=active".to_string()).unwrap();
    
    assert_eq!(query.parameters.0.len(), 3);
    
    // Normal parameters (repeated)
    assert!(query.parameters.0.contains_key("name"));
    let (similarity, values) = &query.parameters.0["name"];
    assert_eq!(*similarity, Similarity::Equals);
    assert_eq!(*values, vec!["ben", "john"]);
    
    // Similarity-based parameter
    assert!(query.parameters.0.contains_key("age"));
    let (similarity, values) = &query.parameters.0["age"];
    assert_eq!(*similarity, Similarity::Contains);
    assert_eq!(*values, vec!["20"]);
    
    // Normal parameter (single)
    assert!(query.parameters.0.contains_key("status"));
    let (similarity, values) = &query.parameters.0["status"];
    assert_eq!(*similarity, Similarity::Equals);
    assert_eq!(*values, vec!["active"]);
}

#[test]
fn test_query_from_http_normal_with_special_params() {
    let query = Query::from_http("name=ben&age=20&order=date_created:desc&limit=25&offset=10".to_string()).unwrap();
    
    assert_eq!(query.parameters.0.len(), 2);
    
    assert!(query.parameters.0.contains_key("name"));
    let (similarity, values) = &query.parameters.0["name"];
    assert_eq!(*similarity, Similarity::Equals);
    assert_eq!(*values, vec!["ben"]);
    
    assert!(query.parameters.0.contains_key("age"));
    let (similarity, values) = &query.parameters.0["age"];
    assert_eq!(*similarity, Similarity::Equals);
    assert_eq!(*values, vec!["20"]);
    
    // Check special parameters are handled correctly
    assert_eq!(query.sort_fields.0.len(), 1);
    assert_eq!(query.sort_fields.0.get("date_created"), Some(&SortOrder::Descending));
    assert_eq!(query.limit, 25);
    assert_eq!(query.offset, 10);
}

#[test]
fn test_query_from_http_url_encoded_normal_params() {
    let query = Query::from_http("name=john%20doe&email=test%40example.com".to_string()).unwrap();
    
    assert_eq!(query.parameters.0.len(), 2);
    
    assert!(query.parameters.0.contains_key("name"));
    let (similarity, values) = &query.parameters.0["name"];
    assert_eq!(*similarity, Similarity::Equals);
    assert_eq!(*values, vec!["john doe"]);
    
    assert!(query.parameters.0.contains_key("email"));
    let (similarity, values) = &query.parameters.0["email"];
    assert_eq!(*similarity, Similarity::Equals);
    assert_eq!(*values, vec!["test@example.com"]);
}

#[test]
fn test_query_from_http_repeated_mixed_similarity() {
    let query = Query::from_http("name=ben&name=contains:john&name=alice".to_string()).unwrap();
    
    assert_eq!(query.parameters.0.len(), 1);
    
    assert!(query.parameters.0.contains_key("name"));
    let (similarity, values) = &query.parameters.0["name"];
    // The similarity-based parameter takes precedence
    assert_eq!(*similarity, Similarity::Contains);
    assert_eq!(*values, vec!["john"]);
}

#[test]
fn test_query_from_http_empty_normal_values() {
    let query = Query::from_http("name=&age=20&status=".to_string()).unwrap();
    
    assert_eq!(query.parameters.0.len(), 1);
    
    assert!(query.parameters.0.contains_key("age"));
    let (similarity, values) = &query.parameters.0["age"];
    assert_eq!(*similarity, Similarity::Equals);
    assert_eq!(*values, vec!["20"]);
}

#[test]
fn test_query_to_http_normal_parameters() {
    let mut query = Query::new();
    
    query.parameters.0.insert("name".to_string(), (Similarity::Equals, vec!["ben".to_string(), "john".to_string()]));
    query.parameters.0.insert("age".to_string(), (Similarity::Equals, vec!["20".to_string()]));
    
    let http = query.to_http();
    
    assert!(http.contains("name=equals:ben,john"));
    assert!(http.contains("age=equals:20"));
    assert!(http.contains("limit=50"));
    assert!(http.contains("offset=0"));
}

#[test]
fn test_query_roundtrip_normal_parameters() {
    let original = "name=ben&name=john&age=20&status=active";
    let query = Query::from_http(original.to_string()).unwrap();
    let reconstructed = query.to_http();
    
    // The reconstructed query should contain the same information
    assert!(reconstructed.contains("name=equals:ben,john"));
    assert!(reconstructed.contains("age=equals:20"));
    assert!(reconstructed.contains("status=equals:active"));
    assert!(reconstructed.contains("limit=50"));
    assert!(reconstructed.contains("offset=0"));
}

#[test]
fn test_query_roundtrip_mixed_normal_and_similarity() {
    let original = "name=ben&name=john&age=contains:20&status=active";
    let query = Query::from_http(original.to_string()).unwrap();
    let reconstructed = query.to_http();
    
    // The reconstructed query should contain the same information
    assert!(reconstructed.contains("name=equals:ben,john"));
    assert!(reconstructed.contains("age=contains:20"));
    assert!(reconstructed.contains("status=equals:active"));
    assert!(reconstructed.contains("limit=50"));
    assert!(reconstructed.contains("offset=0"));
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_normal_parameters() {
    let mut query = Query::new();
    
    query.parameters.0.insert("name".to_string(), (Similarity::Equals, vec!["ben".to_string(), "john".to_string()]));
    query.parameters.0.insert("age".to_string(), (Similarity::Equals, vec!["20".to_string()]));
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name IN (?, ?) AND age = ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_mixed_normal_and_similarity() {
    let mut query = Query::new();
    
    query.parameters.0.insert("name".to_string(), (Similarity::Equals, vec!["ben".to_string(), "john".to_string()]));
    query.parameters.0.insert("age".to_string(), (Similarity::Contains, vec!["20".to_string()]));
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name IN (?, ?) AND age LIKE ? LIMIT ? OFFSET ?");
}
