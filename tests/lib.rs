use query_lite::error::Error;
use query_lite::*;
use std::str::FromStr;

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
    fields.ascending("name".to_string());

    assert_eq!(fields.0.len(), 1);
    assert_eq!(fields.0.get("name"), Some(&SortOrder::Ascending));
}

#[test]
fn test_sort_fields_helper_functions_desc() {
    let mut fields = SortFields::new();
    fields.descending("date_created".to_string());

    assert_eq!(fields.0.len(), 1);
    assert_eq!(fields.0.get("date_created"), Some(&SortOrder::Descending));
}

#[test]
fn test_sort_fields_helper_functions_fluent_api() {
    let mut fields = SortFields::new();
    fields
        .ascending("name".to_string())
        .descending("date_created".to_string())
        .ascending("email".to_string());

    assert_eq!(fields.0.len(), 3);
    assert_eq!(fields.0.get("name"), Some(&SortOrder::Ascending));
    assert_eq!(fields.0.get("date_created"), Some(&SortOrder::Descending));
    assert_eq!(fields.0.get("email"), Some(&SortOrder::Ascending));
}

#[test]
fn test_sort_fields_helper_functions_overwrite() {
    let mut fields = SortFields::new();
    fields.ascending("name".to_string());
    fields.descending("name".to_string()); // Should overwrite

    assert_eq!(fields.0.len(), 1);
    assert_eq!(fields.0.get("name"), Some(&SortOrder::Descending));
}

#[test]
fn test_sort_fields_helper_functions_empty_name() {
    let mut fields = SortFields::new();
    fields.ascending("".to_string());

    assert_eq!(fields.0.len(), 1);
    assert_eq!(fields.0.get(""), Some(&SortOrder::Ascending));
}

#[test]
fn test_sort_fields_helper_functions_whitespace_name() {
    let mut fields = SortFields::new();
    fields.ascending("  name  ".to_string());

    assert_eq!(fields.0.len(), 1);
    assert_eq!(fields.0.get("  name  "), Some(&SortOrder::Ascending));
}

#[test]
fn test_sort_fields_helper_functions_special_characters() {
    let mut fields = SortFields::new();
    fields.ascending("user_name_123".to_string());

    assert_eq!(fields.0.len(), 1);
    assert_eq!(fields.0.get("user_name_123"), Some(&SortOrder::Ascending));
}

#[test]
fn test_sort_fields_helper_functions_unicode_name() {
    let mut fields = SortFields::new();
    fields.ascending("用户_姓名".to_string());

    assert_eq!(fields.0.len(), 1);
    assert_eq!(fields.0.get("用户_姓名"), Some(&SortOrder::Ascending));
}

#[test]
fn test_sort_fields_keep() {
    let mut fields = SortFields::new();
    fields.ascending("name".to_string());
    fields.descending("date_created".to_string());
    fields.ascending("email".to_string());

    let filtered = fields.keep(vec!["name".to_string(), "email".to_string()]);

    assert_eq!(filtered.0.len(), 2);
    assert_eq!(filtered.0.get("name"), Some(&SortOrder::Ascending));
    assert_eq!(filtered.0.get("date_created"), None);
    assert_eq!(filtered.0.get("email"), Some(&SortOrder::Ascending));
}

#[test]
fn test_sort_fields_keep_nonexistent_keys() {
    let mut fields = SortFields::new();
    fields.ascending("name".to_string());

    let filtered = fields.keep(vec!["nonexistent".to_string()]);
    assert_eq!(filtered.0.len(), 0);
}

#[test]
fn test_sort_fields_keep_empty_keys() {
    let mut fields = SortFields::new();
    fields.ascending("name".to_string());

    let filtered = fields.keep(vec![]);
    assert_eq!(filtered.0.len(), 0);
}

#[test]
fn test_sort_fields_remove() {
    let mut fields = SortFields::new();
    fields.ascending("name".to_string());
    fields.descending("date_created".to_string());
    fields.ascending("email".to_string());

    let filtered = fields.remove(vec!["name".to_string(), "email".to_string()]);

    assert_eq!(filtered.0.len(), 1);
    assert_eq!(filtered.0.get("name"), None);
    assert_eq!(filtered.0.get("date_created"), Some(&SortOrder::Descending));
    assert_eq!(filtered.0.get("email"), None);
}

#[test]
fn test_sort_fields_remove_nonexistent_keys() {
    let mut fields = SortFields::new();
    fields.ascending("name".to_string());

    let filtered = fields.remove(vec!["nonexistent".to_string()]);
    assert_eq!(filtered.0.len(), 1);
    assert_eq!(filtered.0.get("name"), Some(&SortOrder::Ascending));
}

#[test]
fn test_sort_fields_remove_empty_keys() {
    let mut fields = SortFields::new();
    fields.ascending("name".to_string());

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
    assert_eq!(Similarity::from_str("lesser").unwrap(), Similarity::Lesser);
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
    params
        .equals("name".to_string(), vec!["damian".to_string()])
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
    let params =
        Parameters::from_str("age=between:20,30&price=greater:100&score=lesser-or-equal:85")
            .unwrap();
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
    let params = Parameters::from_str(
        "name=contains:damian&age=between:25,35&price=greater-or-equal:50&status=equals:active",
    )
    .unwrap();
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
    let params =
        Parameters::from_str("  age  =  between:20,30  &  price  =  lesser:100  ").unwrap();
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
    query.parameters.0.insert(
        "name".to_string(),
        (Similarity::Contains, vec!["damian".to_string()]),
    );

    let http = query.to_http();
    assert!(http.contains("name=contains:damian"));
    assert!(http.contains("limit=50"));
    assert!(http.contains("offset=0"));
}

#[test]
fn test_query_to_http_with_sort() {
    let mut query = Query::new();
    query.sort_fields.descending("date_created".to_string());

    let http = query.to_http();
    assert!(http.contains("date_created:desc"));
    assert!(http.contains("limit=50"));
    assert!(http.contains("offset=0"));
}

#[test]
fn test_query_to_http_with_params_and_sort() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "name".to_string(),
        (Similarity::Contains, vec!["damian".to_string()]),
    );

    query.sort_fields.descending("date_created".to_string());

    let http = query.to_http();
    assert!(http.contains("name=contains:damian"));
    assert!(http.contains("date_created:desc"));
    assert!(http.contains("limit=50"));
    assert!(http.contains("offset=0"));
}

#[test]
fn test_query_to_http_sort_fields_empty_values() {
    let mut query = Query::new();
    query
        .parameters
        .0
        .insert("name".to_string(), (Similarity::Contains, vec![]));

    let http = query.to_http();
    assert!(!http.contains("name="));
    assert_eq!(http, "limit=50&offset=0");
}

#[test]
fn test_query_to_http_empty_sort_fields() {
    let mut query = Query::new();
    query.sort_fields.ascending("".to_string());

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
    assert_eq!(
        query.sort_fields.0.get("date_created"),
        Some(&SortOrder::Descending)
    );
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
    assert_eq!(
        query.sort_fields.0.get("date_created"),
        Some(&SortOrder::Descending)
    );

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
    assert_eq!(
        query.sort_fields.0.get("date_created"),
        Some(&SortOrder::Descending)
    );
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
    params.0.insert(
        "name".to_string(),
        (Similarity::Contains, vec!["value1".to_string()]),
    );
    params.0.insert(
        "surname".to_string(),
        (Similarity::Equals, vec!["value2".to_string()]),
    );
    params.0.insert(
        "email".to_string(),
        (Similarity::StartsWith, vec!["value3".to_string()]),
    );

    let filtered = params.keep(vec!["name".to_string(), "email".to_string()]);

    assert_eq!(filtered.0.len(), 2);
    assert!(filtered.0.contains_key("name"));
    assert!(!filtered.0.contains_key("surname"));
    assert!(filtered.0.contains_key("email"));
}

#[test]
fn test_parameters_keep_nonexistent_keys() {
    let mut params = Parameters::new();
    params.0.insert(
        "name".to_string(),
        (Similarity::Contains, vec!["value1".to_string()]),
    );

    let filtered = params.keep(vec!["nonexistent".to_string()]);
    assert_eq!(filtered.0.len(), 0);
}

#[test]
fn test_parameters_keep_empty_keys() {
    let mut params = Parameters::new();
    params.0.insert(
        "name".to_string(),
        (Similarity::Contains, vec!["value1".to_string()]),
    );

    let filtered = params.keep(vec![]);
    assert_eq!(filtered.0.len(), 0);
}

#[test]
fn test_parameters_remove() {
    let mut params = Parameters::new();
    params.0.insert(
        "name".to_string(),
        (Similarity::Contains, vec!["value1".to_string()]),
    );
    params.0.insert(
        "surname".to_string(),
        (Similarity::Equals, vec!["value2".to_string()]),
    );
    params.0.insert(
        "email".to_string(),
        (Similarity::StartsWith, vec!["value3".to_string()]),
    );

    let filtered = params.remove(vec!["name".to_string(), "email".to_string()]);

    assert_eq!(filtered.0.len(), 1);
    assert!(!filtered.0.contains_key("name"));
    assert!(filtered.0.contains_key("surname"));
    assert!(!filtered.0.contains_key("email"));
}

#[test]
fn test_parameters_remove_nonexistent_keys() {
    let mut params = Parameters::new();
    params.0.insert(
        "name".to_string(),
        (Similarity::Contains, vec!["value1".to_string()]),
    );

    let filtered = params.remove(vec!["nonexistent".to_string()]);
    assert_eq!(filtered.0.len(), 1);
    assert!(filtered.0.contains_key("name"));
}

#[test]
fn test_parameters_remove_empty_keys() {
    let mut params = Parameters::new();
    params.0.insert(
        "name".to_string(),
        (Similarity::Contains, vec!["value1".to_string()]),
    );

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
    assert_eq!(
        query.sort_fields.0.get("date_created"),
        Some(&SortOrder::Descending)
    );

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

    query.parameters.0.insert(
        "age".to_string(),
        (
            Similarity::Between,
            vec!["20".to_string(), "30".to_string()],
        ),
    );
    query.parameters.0.insert(
        "price".to_string(),
        (Similarity::Greater, vec!["100".to_string()]),
    );
    query.parameters.0.insert(
        "score".to_string(),
        (Similarity::LesserOrEqual, vec!["85".to_string()]),
    );

    query.sort_fields.descending("date_created".to_string());

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

    params.0.insert(
        "age".to_string(),
        (
            Similarity::Between,
            vec!["20".to_string(), "30".to_string()],
        ),
    );
    params.0.insert(
        "price".to_string(),
        (Similarity::Greater, vec!["100".to_string()]),
    );
    params.0.insert(
        "name".to_string(),
        (Similarity::Contains, vec!["damian".to_string()]),
    );

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

    params.0.insert(
        "age".to_string(),
        (
            Similarity::Between,
            vec!["20".to_string(), "30".to_string()],
        ),
    );
    params.0.insert(
        "price".to_string(),
        (Similarity::Greater, vec!["100".to_string()]),
    );
    params.0.insert(
        "name".to_string(),
        (Similarity::Contains, vec!["damian".to_string()]),
    );

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
    assert_eq!(
        query.sort_fields.0.get("date_created"),
        Some(&SortOrder::Descending)
    );
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
    let query_str =
        "  age  =  between:20,30  &  price  =  greater:100  &  score  =  lesser-or-equal:85  ";
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
    query.parameters.0.insert(
        "name".to_string(),
        (Similarity::Equals, vec!["damian".to_string()]),
    );

    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name = ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_equals_multiple_values() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "name".to_string(),
        (
            Similarity::Equals,
            vec!["damian".to_string(), "john".to_string()],
        ),
    );

    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name IN (?, ?) LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_equals_null() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "name".to_string(),
        (Similarity::Equals, vec!["null".to_string()]),
    );

    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name IS ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_contains() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "name".to_string(),
        (Similarity::Contains, vec!["damian".to_string()]),
    );

    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name LIKE ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_contains_multiple_values() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "name".to_string(),
        (
            Similarity::Contains,
            vec!["damian".to_string(), "john".to_string()],
        ),
    );

    let sql = query.to_sql();
    assert_eq!(sql, "WHERE (name LIKE ? OR name LIKE ?) LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_starts_with() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "name".to_string(),
        (Similarity::StartsWith, vec!["damian".to_string()]),
    );

    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name LIKE ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_ends_with() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "name".to_string(),
        (Similarity::EndsWith, vec!["damian".to_string()]),
    );

    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name LIKE ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_between() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "age".to_string(),
        (
            Similarity::Between,
            vec!["20".to_string(), "30".to_string()],
        ),
    );

    let sql = query.to_sql();
    assert_eq!(sql, "WHERE age BETWEEN ? AND ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_lesser() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "price".to_string(),
        (Similarity::Lesser, vec!["100".to_string()]),
    );

    let sql = query.to_sql();
    assert_eq!(sql, "WHERE price < ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_lesser_or_equal() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "price".to_string(),
        (Similarity::LesserOrEqual, vec!["100".to_string()]),
    );

    let sql = query.to_sql();
    assert_eq!(sql, "WHERE price <= ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_greater() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "price".to_string(),
        (Similarity::Greater, vec!["50".to_string()]),
    );

    let sql = query.to_sql();
    assert_eq!(sql, "WHERE price > ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_greater_or_equal() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "price".to_string(),
        (Similarity::GreaterOrEqual, vec!["50".to_string()]),
    );

    let sql = query.to_sql();
    assert_eq!(sql, "WHERE price >= ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_order_by() {
    let mut query = Query::new();
    query.sort_fields.descending("date_created".to_string());

    let sql = query.to_sql();
    assert_eq!(sql, "ORDER BY date_created DESC LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_multiple_order_by() {
    let mut query = Query::new();
    query.sort_fields.descending("date_created".to_string());
    query.sort_fields.ascending("name".to_string());

    let sql = query.to_sql();
    assert_eq!(sql, "ORDER BY date_created DESC, name ASC LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_complex() {
    let mut query = Query::new();

    // Add multiple parameters
    query.parameters.0.insert(
        "name".to_string(),
        (Similarity::Contains, vec!["damian".to_string()]),
    );
    query.parameters.0.insert(
        "age".to_string(),
        (
            Similarity::Between,
            vec!["20".to_string(), "30".to_string()],
        ),
    );
    query.parameters.0.insert(
        "price".to_string(),
        (Similarity::Greater, vec!["100".to_string()]),
    );

    // Add sorting
    query.sort_fields.descending("date_created".to_string());

    let sql = query.to_sql();
    assert_eq!(
        sql,
        "WHERE name LIKE ? AND age BETWEEN ? AND ? AND price > ? ORDER BY date_created DESC LIMIT ? OFFSET ?"
    );
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_empty_parameters() {
    let mut query = Query::new();
    query
        .parameters
        .0
        .insert("name".to_string(), (Similarity::Contains, vec![]));

    let sql = query.to_sql();
    assert_eq!(sql, "LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_empty_sort_fields() {
    let mut query = Query::new();
    query.sort_fields.ascending("".to_string());

    let sql = query.to_sql();
    assert_eq!(sql, "LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_numeric_comparisons_multiple_values() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "price".to_string(),
        (
            Similarity::Greater,
            vec!["50".to_string(), "100".to_string()],
        ),
    );

    let sql = query.to_sql();
    assert_eq!(sql, "WHERE (price > ? OR price > ?) LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_between_invalid_values() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "age".to_string(),
        (Similarity::Between, vec!["20".to_string()]),
    );

    let sql = query.to_sql();
    assert_eq!(sql, "LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_between_multiple_pairs() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "age".to_string(),
        (
            Similarity::Between,
            vec![
                "10".to_string(),
                "20".to_string(),
                "30".to_string(),
                "40".to_string(),
            ],
        ),
    );

    let sql = query.to_sql();
    assert_eq!(
        sql,
        "WHERE (age BETWEEN ? AND ? OR age BETWEEN ? AND ?) LIMIT ? OFFSET ?"
    );
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_between_odd_values_ignored() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "age".to_string(),
        (
            Similarity::Between,
            vec![
                "10".to_string(),
                "20".to_string(),
                "30".to_string(),
                "40".to_string(),
                "50".to_string(),
            ],
        ),
    );

    let sql = query.to_sql();
    assert_eq!(
        sql,
        "WHERE (age BETWEEN ? AND ? OR age BETWEEN ? AND ?) LIMIT ? OFFSET ?"
    );
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_between_three_pairs() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "age".to_string(),
        (
            Similarity::Between,
            vec![
                "10".to_string(),
                "20".to_string(),
                "30".to_string(),
                "40".to_string(),
                "50".to_string(),
                "60".to_string(),
            ],
        ),
    );

    let sql = query.to_sql();
    assert_eq!(
        sql,
        "WHERE (age BETWEEN ? AND ? OR age BETWEEN ? AND ? OR age BETWEEN ? AND ?) LIMIT ? OFFSET ?"
    );
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_between_single_pair() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "age".to_string(),
        (
            Similarity::Between,
            vec!["20".to_string(), "30".to_string()],
        ),
    );

    let sql = query.to_sql();
    assert_eq!(sql, "WHERE age BETWEEN ? AND ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_between_empty_values() {
    let mut query = Query::new();
    query
        .parameters
        .0
        .insert("age".to_string(), (Similarity::Between, vec![]));

    let sql = query.to_sql();
    assert_eq!(sql, "LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_between_complex_with_other_conditions() {
    let mut query = Query::new();

    // Add between with multiple pairs
    query.parameters.0.insert(
        "age".to_string(),
        (
            Similarity::Between,
            vec![
                "10".to_string(),
                "20".to_string(),
                "30".to_string(),
                "40".to_string(),
                "50".to_string(),
            ],
        ),
    );

    // Add other condition
    query.parameters.0.insert(
        "name".to_string(),
        (Similarity::Contains, vec!["damian".to_string()]),
    );

    let sql = query.to_sql();
    assert_eq!(
        sql,
        "WHERE (age BETWEEN ? AND ? OR age BETWEEN ? AND ?) AND name LIKE ? LIMIT ? OFFSET ?"
    );
}

// ============================================================================
// PARAMETER TYPE ALIAS AND TRAIT TESTS
// ============================================================================

#[test]
fn test_parameter_type_alias() {
    // Test that Parameter is correctly defined as a tuple type alias
    let param: Parameter = (Similarity::Contains, vec!["test".to_string()]);

    // Test tuple access (backward compatibility)
    assert_eq!(param.0, Similarity::Contains);
    assert_eq!(param.1, vec!["test"]);
}

#[test]
fn test_parameter_get_trait_similarity() {
    let param: Parameter = (
        Similarity::Equals,
        vec!["value1".to_string(), "value2".to_string()],
    );

    // Test trait method access
    assert_eq!(param.similarity(), &Similarity::Equals);

    // Test with different similarity types
    let param2: Parameter = (
        Similarity::Between,
        vec!["10".to_string(), "20".to_string()],
    );
    assert_eq!(param2.similarity(), &Similarity::Between);

    let param3: Parameter = (Similarity::GreaterOrEqual, vec!["100".to_string()]);
    assert_eq!(param3.similarity(), &Similarity::GreaterOrEqual);
}

#[test]
fn test_parameter_get_trait_values() {
    let param: Parameter = (
        Similarity::Contains,
        vec!["value1".to_string(), "value2".to_string()],
    );

    // Test trait method access
    assert_eq!(
        param.values(),
        &vec!["value1".to_string(), "value2".to_string()]
    );

    // Test with empty values
    let param2: Parameter = (Similarity::Equals, vec![]);
    assert_eq!(param2.values(), &vec![] as &Vec<String>);

    // Test with single value
    let param3: Parameter = (Similarity::StartsWith, vec!["prefix".to_string()]);
    assert_eq!(param3.values(), &vec!["prefix".to_string()]);
}

#[test]
fn test_parameter_get_trait_all_similarity_types() {
    let similarities = vec![
        Similarity::Equals,
        Similarity::Contains,
        Similarity::StartsWith,
        Similarity::EndsWith,
        Similarity::Between,
        Similarity::Lesser,
        Similarity::LesserOrEqual,
        Similarity::Greater,
        Similarity::GreaterOrEqual,
    ];

    for similarity in similarities {
        let param: Parameter = (similarity.clone(), vec!["test".to_string()]);
        assert_eq!(param.similarity(), &similarity);
        assert_eq!(param.values(), &vec!["test".to_string()]);
    }
}

#[test]
fn test_parameter_get_trait_immutable_references() {
    let param: Parameter = (
        Similarity::Contains,
        vec!["value1".to_string(), "value2".to_string()],
    );

    // Test that we get immutable references
    let similarity_ref = param.similarity();
    let values_ref = param.values();

    // These should be immutable references
    assert_eq!(*similarity_ref, Similarity::Contains);
    assert_eq!(
        *values_ref,
        vec!["value1".to_string(), "value2".to_string()]
    );

    // Test that we can get multiple references (no ownership issues)
    let similarity_ref2 = param.similarity();
    let values_ref2 = param.values();

    assert_eq!(similarity_ref, similarity_ref2);
    assert_eq!(values_ref, values_ref2);
}

#[test]
fn test_parameter_get_trait_with_complex_values() {
    let complex_values = vec![
        "value with spaces".to_string(),
        "value,with,commas".to_string(),
        "value%20encoded".to_string(),
        "value\nwith\nnewlines".to_string(),
        "value\twith\ttabs".to_string(),
    ];

    let param: Parameter = (Similarity::Contains, complex_values.clone());

    assert_eq!(param.similarity(), &Similarity::Contains);
    assert_eq!(param.values(), &complex_values);
}

// ============================================================================
// INNER METHODS TESTS
// ============================================================================

#[test]
fn test_parameters_inner_method() {
    let mut params = Parameters::new();
    params.equals("name".to_string(), vec!["test".to_string()]);
    params.contains("description".to_string(), vec!["rust".to_string()]);

    // Test inner() method returns reference to IndexMap
    let inner_map = params.inner();
    assert_eq!(inner_map.len(), 2);
    assert!(inner_map.contains_key("name"));
    assert!(inner_map.contains_key("description"));

    // Test that we can access the underlying data
    let name_param = inner_map.get("name").unwrap();
    assert_eq!(name_param.0, Similarity::Equals);
    assert_eq!(name_param.1, vec!["test"]);

    let desc_param = inner_map.get("description").unwrap();
    assert_eq!(desc_param.0, Similarity::Contains);
    assert_eq!(desc_param.1, vec!["rust"]);
}

#[test]
fn test_parameters_inner_mut_method() {
    let mut params = Parameters::new();
    params.equals("name".to_string(), vec!["test".to_string()]);

    // Test inner_mut() method returns mutable reference
    let inner_map = params.inner_mut();
    assert_eq!(inner_map.len(), 1);

    // Test that we can modify the underlying data
    inner_map.insert(
        "new_key".to_string(),
        (
            Similarity::Between,
            vec!["10".to_string(), "20".to_string()],
        ),
    );

    // Verify the change
    assert_eq!(inner_map.len(), 2);
    assert!(inner_map.contains_key("new_key"));

    let new_param = inner_map.get("new_key").unwrap();
    assert_eq!(new_param.0, Similarity::Between);
    assert_eq!(new_param.1, vec!["10", "20"]);
}

#[test]
fn test_sort_fields_inner_method() {
    let mut sort_fields = SortFields::new();
    sort_fields.ascending("name".to_string());
    sort_fields.descending("date_created".to_string());

    // Test inner() method returns reference to IndexMap
    let inner_map = sort_fields.inner();
    assert_eq!(inner_map.len(), 2);
    assert!(inner_map.contains_key("name"));
    assert!(inner_map.contains_key("date_created"));

    // Test that we can access the underlying data
    assert_eq!(inner_map.get("name"), Some(&SortOrder::Ascending));
    assert_eq!(inner_map.get("date_created"), Some(&SortOrder::Descending));
}

#[test]
fn test_sort_fields_inner_mut_method() {
    let mut sort_fields = SortFields::new();
    sort_fields.ascending("name".to_string());

    // Test inner_mut() method returns mutable reference
    let inner_map = sort_fields.inner_mut();
    assert_eq!(inner_map.len(), 1);

    // Test that we can modify the underlying data
    inner_map.insert("new_field".to_string(), SortOrder::Descending);

    // Verify the change
    assert_eq!(inner_map.len(), 2);
    assert!(inner_map.contains_key("new_field"));
    assert_eq!(inner_map.get("new_field"), Some(&SortOrder::Descending));
}

#[test]
fn test_inner_methods_preserve_order() {
    let mut params = Parameters::new();
    params.equals("first".to_string(), vec!["1".to_string()]);
    params.contains("second".to_string(), vec!["2".to_string()]);
    params.between("third".to_string(), vec!["3".to_string(), "4".to_string()]);

    // Test that inner() preserves insertion order
    let inner_map = params.inner();
    let keys: Vec<&String> = inner_map.keys().collect();
    assert_eq!(keys, vec!["first", "second", "third"]);

    // Test that inner_mut() preserves order after modification
    let inner_map_mut = params.inner_mut();
    inner_map_mut.insert(
        "fourth".to_string(),
        (Similarity::Greater, vec!["5".to_string()]),
    );

    let keys_after: Vec<&String> = inner_map_mut.keys().collect();
    assert_eq!(keys_after, vec!["first", "second", "third", "fourth"]);
}

#[test]
fn test_inner_methods_with_empty_collections() {
    let params = Parameters::new();
    let sort_fields = SortFields::new();

    // Test inner() with empty collections
    assert_eq!(params.inner().len(), 0);
    assert_eq!(sort_fields.inner().len(), 0);

    // Test inner_mut() with empty collections
    let mut params_mut = Parameters::new();
    let mut sort_fields_mut = SortFields::new();

    assert_eq!(params_mut.inner_mut().len(), 0);
    assert_eq!(sort_fields_mut.inner_mut().len(), 0);
}

// ============================================================================
// METHOD RENAME TESTS (ascending/descending)
// ============================================================================

#[test]
fn test_sort_fields_ascending_method() {
    let mut sort_fields = SortFields::new();
    sort_fields.ascending("name".to_string());

    assert_eq!(sort_fields.0.len(), 1);
    assert_eq!(sort_fields.0.get("name"), Some(&SortOrder::Ascending));
}

#[test]
fn test_sort_fields_descending_method() {
    let mut sort_fields = SortFields::new();
    sort_fields.descending("date_created".to_string());

    assert_eq!(sort_fields.0.len(), 1);
    assert_eq!(
        sort_fields.0.get("date_created"),
        Some(&SortOrder::Descending)
    );
}

#[test]
fn test_sort_fields_ascending_descending_fluent_api() {
    let mut sort_fields = SortFields::new();
    sort_fields
        .ascending("name".to_string())
        .descending("date_created".to_string())
        .ascending("email".to_string());

    assert_eq!(sort_fields.0.len(), 3);
    assert_eq!(sort_fields.0.get("name"), Some(&SortOrder::Ascending));
    assert_eq!(
        sort_fields.0.get("date_created"),
        Some(&SortOrder::Descending)
    );
    assert_eq!(sort_fields.0.get("email"), Some(&SortOrder::Ascending));
}

#[test]
fn test_sort_fields_ascending_descending_overwrite() {
    let mut sort_fields = SortFields::new();
    sort_fields.ascending("name".to_string());
    sort_fields.descending("name".to_string()); // Should overwrite

    assert_eq!(sort_fields.0.len(), 1);
    assert_eq!(sort_fields.0.get("name"), Some(&SortOrder::Descending));
}

#[test]
fn test_sort_fields_ascending_descending_with_special_characters() {
    let mut sort_fields = SortFields::new();
    sort_fields.ascending("user_name_123".to_string());
    sort_fields.descending("date_created_at".to_string());

    assert_eq!(sort_fields.0.len(), 2);
    assert_eq!(
        sort_fields.0.get("user_name_123"),
        Some(&SortOrder::Ascending)
    );
    assert_eq!(
        sort_fields.0.get("date_created_at"),
        Some(&SortOrder::Descending)
    );
}

#[test]
fn test_sort_fields_ascending_descending_with_unicode() {
    let mut sort_fields = SortFields::new();
    sort_fields.ascending("用户_姓名".to_string());
    sort_fields.descending("创建_日期".to_string());

    assert_eq!(sort_fields.0.len(), 2);
    assert_eq!(sort_fields.0.get("用户_姓名"), Some(&SortOrder::Ascending));
    assert_eq!(sort_fields.0.get("创建_日期"), Some(&SortOrder::Descending));
}

#[test]
fn test_sort_fields_ascending_descending_empty_strings() {
    let mut sort_fields = SortFields::new();
    sort_fields.ascending("".to_string());
    sort_fields.descending("  ".to_string());

    assert_eq!(sort_fields.0.len(), 2);
    assert_eq!(sort_fields.0.get(""), Some(&SortOrder::Ascending));
    assert_eq!(sort_fields.0.get("  "), Some(&SortOrder::Descending));
}

// ============================================================================
// INTEGRATION TESTS FOR NEW FEATURES
// ============================================================================

#[test]
fn test_parameter_trait_with_parameters_collection() {
    let mut params = Parameters::new();
    params.equals(
        "name".to_string(),
        vec!["john".to_string(), "jane".to_string()],
    );
    params.contains("description".to_string(), vec!["rust".to_string()]);

    // Test accessing parameters using the trait methods
    let name_param = params.inner().get("name").unwrap();
    assert_eq!(name_param.similarity(), &Similarity::Equals);
    assert_eq!(
        name_param.values(),
        &vec!["john".to_string(), "jane".to_string()]
    );

    let desc_param = params.inner().get("description").unwrap();
    assert_eq!(desc_param.similarity(), &Similarity::Contains);
    assert_eq!(desc_param.values(), &vec!["rust".to_string()]);
}

#[test]
fn test_parameter_trait_with_query_parsing() {
    let query = Query::from_http("name=contains:john&age=between:20,30".to_string()).unwrap();

    // Test accessing parsed parameters using the trait methods
    let name_param = query.parameters.inner().get("name").unwrap();
    assert_eq!(name_param.similarity(), &Similarity::Contains);
    assert_eq!(name_param.values(), &vec!["john".to_string()]);

    let age_param = query.parameters.inner().get("age").unwrap();
    assert_eq!(age_param.similarity(), &Similarity::Between);
    assert_eq!(
        age_param.values(),
        &vec!["20".to_string(), "30".to_string()]
    );
}

#[test]
fn test_inner_methods_with_query_manipulation() {
    let mut query = Query::new();
    query
        .parameters
        .equals("name".to_string(), vec!["john".to_string()]);
    query
        .parameters
        .contains("description".to_string(), vec!["rust".to_string()]);
    query.sort_fields.ascending("name".to_string());
    query.sort_fields.descending("date_created".to_string());

    // Test using inner() for complex operations
    let param_map = query.parameters.inner();
    let sort_map = query.sort_fields.inner();

    // Verify we can iterate and access data
    let param_keys: Vec<&String> = param_map.keys().collect();
    let sort_keys: Vec<&String> = sort_map.keys().collect();

    assert_eq!(param_keys, vec!["name", "description"]);
    assert_eq!(sort_keys, vec!["name", "date_created"]);

    // Test using inner_mut() for modifications
    let param_map_mut = query.parameters.inner_mut();
    let sort_map_mut = query.sort_fields.inner_mut();

    param_map_mut.insert(
        "new_param".to_string(),
        (Similarity::Greater, vec!["100".to_string()]),
    );
    sort_map_mut.insert("new_sort".to_string(), SortOrder::Ascending);

    // Verify modifications
    assert_eq!(param_map_mut.len(), 3);
    assert_eq!(sort_map_mut.len(), 3);
    assert!(param_map_mut.contains_key("new_param"));
    assert!(sort_map_mut.contains_key("new_sort"));
}

#[test]
fn test_renamed_methods_with_query_building() {
    let mut params = Parameters::new();
    let mut sort_fields = SortFields::new();

    // Build using new method names
    params
        .equals("name".to_string(), vec!["john".to_string()])
        .contains("description".to_string(), vec!["rust".to_string()]);

    sort_fields
        .ascending("name".to_string())
        .descending("date_created".to_string());

    let query = Query::init(params, sort_fields, 25, 10);

    // Verify the query was built correctly
    assert_eq!(query.parameters.inner().len(), 2);
    assert_eq!(query.sort_fields.inner().len(), 2);
    assert_eq!(query.limit, 25);
    assert_eq!(query.offset, 10);

    // Verify parameter access using trait methods
    let name_param = query.parameters.inner().get("name").unwrap();
    assert_eq!(name_param.similarity(), &Similarity::Equals);
    assert_eq!(name_param.values(), &vec!["john".to_string()]);

    // Verify sort field access
    assert_eq!(
        query.sort_fields.inner().get("name"),
        Some(&SortOrder::Ascending)
    );
    assert_eq!(
        query.sort_fields.inner().get("date_created"),
        Some(&SortOrder::Descending)
    );
}

#[test]
fn test_all_new_features_roundtrip() {
    // Build query using all new features
    let mut params = Parameters::new();
    let mut sort_fields = SortFields::new();

    params
        .equals(
            "name".to_string(),
            vec!["john".to_string(), "jane".to_string()],
        )
        .contains("description".to_string(), vec!["rust".to_string()])
        .between("age".to_string(), vec!["20".to_string(), "30".to_string()]);

    sort_fields
        .ascending("name".to_string())
        .descending("date_created".to_string());

    let query = Query::init(params, sort_fields, 25, 10);

    // Convert to HTTP and back
    let http_string = query.to_http();
    let reconstructed = Query::from_http(http_string).unwrap();

    // Verify all features are preserved
    assert_eq!(reconstructed.parameters.inner().len(), 3);
    assert_eq!(reconstructed.sort_fields.inner().len(), 2);
    assert_eq!(reconstructed.limit, 25);
    assert_eq!(reconstructed.offset, 10);

    // Verify parameter access using trait methods
    let name_param = reconstructed.parameters.inner().get("name").unwrap();
    assert_eq!(name_param.similarity(), &Similarity::Equals);
    assert_eq!(
        name_param.values(),
        &vec!["john".to_string(), "jane".to_string()]
    );

    let desc_param = reconstructed.parameters.inner().get("description").unwrap();
    assert_eq!(desc_param.similarity(), &Similarity::Contains);
    assert_eq!(desc_param.values(), &vec!["rust".to_string()]);

    let age_param = reconstructed.parameters.inner().get("age").unwrap();
    assert_eq!(age_param.similarity(), &Similarity::Between);
    assert_eq!(
        age_param.values(),
        &vec!["20".to_string(), "30".to_string()]
    );

    // Verify sort fields
    assert_eq!(
        reconstructed.sort_fields.inner().get("name"),
        Some(&SortOrder::Ascending)
    );
    assert_eq!(
        reconstructed.sort_fields.inner().get("date_created"),
        Some(&SortOrder::Descending)
    );
}

#[test]
fn test_backward_compatibility_with_new_features() {
    // Test that old tuple access still works alongside new features
    let mut params = Parameters::new();
    params.equals("name".to_string(), vec!["john".to_string()]);

    let param = params.inner().get("name").unwrap();

    // Old tuple access should still work
    assert_eq!(param.0, Similarity::Equals);
    assert_eq!(param.1, vec!["john".to_string()]);

    // New trait access should also work
    assert_eq!(param.similarity(), &Similarity::Equals);
    assert_eq!(param.values(), &vec!["john".to_string()]);

    // Both should return the same values
    assert_eq!(param.0, *param.similarity());
    assert_eq!(param.1, *param.values());
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
    let query =
        Query::from_http("name=ben&name=john&age=contains:20&status=active".to_string()).unwrap();

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
    let query =
        Query::from_http("name=ben&age=20&order=date_created:desc&limit=25&offset=10".to_string())
            .unwrap();

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
    assert_eq!(
        query.sort_fields.0.get("date_created"),
        Some(&SortOrder::Descending)
    );
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

    query.parameters.0.insert(
        "name".to_string(),
        (
            Similarity::Equals,
            vec!["ben".to_string(), "john".to_string()],
        ),
    );
    query.parameters.0.insert(
        "age".to_string(),
        (Similarity::Equals, vec!["20".to_string()]),
    );

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

    query.parameters.0.insert(
        "name".to_string(),
        (
            Similarity::Equals,
            vec!["ben".to_string(), "john".to_string()],
        ),
    );
    query.parameters.0.insert(
        "age".to_string(),
        (Similarity::Equals, vec!["20".to_string()]),
    );

    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name IN (?, ?) AND age = ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_mixed_normal_and_similarity() {
    let mut query = Query::new();

    query.parameters.0.insert(
        "name".to_string(),
        (
            Similarity::Equals,
            vec!["ben".to_string(), "john".to_string()],
        ),
    );
    query.parameters.0.insert(
        "age".to_string(),
        (Similarity::Contains, vec!["20".to_string()]),
    );

    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name IN (?, ?) AND age LIKE ? LIMIT ? OFFSET ?");
}

// ============================================================================
// TO_VALUES TESTS
// ============================================================================

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_empty() {
    let query = Query::new();
    let values = query.to_values().unwrap();

    // Should only contain limit and offset
    assert_eq!(values.len(), 2);
    assert_eq!(values[0], SqlValue::Integer(50)); // default limit
    assert_eq!(values[1], SqlValue::Integer(0)); // default offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_with_custom_limit_offset() {
    let mut query = Query::new();
    query.limit = 100;
    query.offset = 25;

    let values = query.to_values().unwrap();

    assert_eq!(values.len(), 2);
    assert_eq!(values[0], SqlValue::Integer(100));
    assert_eq!(values[1], SqlValue::Integer(25));
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_equals_integer() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "age".to_string(),
        (Similarity::Equals, vec!["25".to_string()]),
    );

    let values = query.to_values().unwrap();

    assert_eq!(values.len(), 3);
    assert_eq!(values[0], SqlValue::Integer(25));
    assert_eq!(values[1], SqlValue::Integer(50)); // limit
    assert_eq!(values[2], SqlValue::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_equals_multiple_integers() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "age".to_string(),
        (
            Similarity::Equals,
            vec!["25".to_string(), "30".to_string(), "35".to_string()],
        ),
    );

    let values = query.to_values().unwrap();

    assert_eq!(values.len(), 5);
    assert_eq!(values[0], SqlValue::Integer(25));
    assert_eq!(values[1], SqlValue::Integer(30));
    assert_eq!(values[2], SqlValue::Integer(35));
    assert_eq!(values[3], SqlValue::Integer(50)); // limit
    assert_eq!(values[4], SqlValue::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_equals_real() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "price".to_string(),
        (Similarity::Equals, vec!["25.5".to_string()]),
    );

    let values = query.to_values().unwrap();

    assert_eq!(values.len(), 3);
    assert_eq!(values[0], SqlValue::Real(25.5));
    assert_eq!(values[1], SqlValue::Integer(50)); // limit
    assert_eq!(values[2], SqlValue::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_equals_text() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "name".to_string(),
        (Similarity::Equals, vec!["john".to_string()]),
    );

    let values = query.to_values().unwrap();

    assert_eq!(values.len(), 3);
    assert_eq!(values[0], SqlValue::Text("john".to_string()));
    assert_eq!(values[1], SqlValue::Integer(50)); // limit
    assert_eq!(values[2], SqlValue::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_equals_null() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "deleted_at".to_string(),
        (Similarity::Equals, vec!["null".to_string()]),
    );

    let values = query.to_values().unwrap();

    assert_eq!(values.len(), 3);
    assert_eq!(values[0], SqlValue::Null);
    assert_eq!(values[1], SqlValue::Integer(50)); // limit
    assert_eq!(values[2], SqlValue::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_contains() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "name".to_string(),
        (Similarity::Contains, vec!["john".to_string()]),
    );

    let values = query.to_values().unwrap();

    assert_eq!(values.len(), 3);
    assert_eq!(values[0], SqlValue::Text("%john%".to_string()));
    assert_eq!(values[1], SqlValue::Integer(50)); // limit
    assert_eq!(values[2], SqlValue::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_contains_multiple() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "name".to_string(),
        (
            Similarity::Contains,
            vec!["john".to_string(), "jane".to_string()],
        ),
    );

    let values = query.to_values().unwrap();

    assert_eq!(values.len(), 4);
    assert_eq!(values[0], SqlValue::Text("%john%".to_string()));
    assert_eq!(values[1], SqlValue::Text("%jane%".to_string()));
    assert_eq!(values[2], SqlValue::Integer(50)); // limit
    assert_eq!(values[3], SqlValue::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_starts_with() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "name".to_string(),
        (Similarity::StartsWith, vec!["john".to_string()]),
    );

    let values = query.to_values().unwrap();

    assert_eq!(values.len(), 3);
    assert_eq!(values[0], SqlValue::Text("john%".to_string()));
    assert_eq!(values[1], SqlValue::Integer(50)); // limit
    assert_eq!(values[2], SqlValue::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_ends_with() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "name".to_string(),
        (Similarity::EndsWith, vec!["son".to_string()]),
    );

    let values = query.to_values().unwrap();

    assert_eq!(values.len(), 3);
    assert_eq!(values[0], SqlValue::Text("%son".to_string()));
    assert_eq!(values[1], SqlValue::Integer(50)); // limit
    assert_eq!(values[2], SqlValue::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_between() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "age".to_string(),
        (
            Similarity::Between,
            vec!["20".to_string(), "30".to_string()],
        ),
    );

    let values = query.to_values().unwrap();

    assert_eq!(values.len(), 4);
    assert_eq!(values[0], SqlValue::Integer(20));
    assert_eq!(values[1], SqlValue::Integer(30));
    assert_eq!(values[2], SqlValue::Integer(50)); // limit
    assert_eq!(values[3], SqlValue::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_lesser() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "price".to_string(),
        (Similarity::Lesser, vec!["100".to_string()]),
    );

    let values = query.to_values().unwrap();

    assert_eq!(values.len(), 3);
    assert_eq!(values[0], SqlValue::Integer(100));
    assert_eq!(values[1], SqlValue::Integer(50)); // limit
    assert_eq!(values[2], SqlValue::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_lesser_or_equal() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "price".to_string(),
        (Similarity::LesserOrEqual, vec!["100".to_string()]),
    );

    let values = query.to_values().unwrap();

    assert_eq!(values.len(), 3);
    assert_eq!(values[0], SqlValue::Integer(100));
    assert_eq!(values[1], SqlValue::Integer(50)); // limit
    assert_eq!(values[2], SqlValue::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_greater() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "price".to_string(),
        (Similarity::Greater, vec!["50".to_string()]),
    );

    let values = query.to_values().unwrap();

    assert_eq!(values.len(), 3);
    assert_eq!(values[0], SqlValue::Integer(50));
    assert_eq!(values[1], SqlValue::Integer(50)); // limit
    assert_eq!(values[2], SqlValue::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_greater_or_equal() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "price".to_string(),
        (Similarity::GreaterOrEqual, vec!["50".to_string()]),
    );

    let values = query.to_values().unwrap();

    assert_eq!(values.len(), 3);
    assert_eq!(values[0], SqlValue::Integer(50));
    assert_eq!(values[1], SqlValue::Integer(50)); // limit
    assert_eq!(values[2], SqlValue::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_mixed_types() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "name".to_string(),
        (Similarity::Contains, vec!["john".to_string()]),
    );
    query.parameters.0.insert(
        "age".to_string(),
        (Similarity::Equals, vec!["25".to_string()]),
    );
    query.parameters.0.insert(
        "price".to_string(),
        (Similarity::Greater, vec!["100.5".to_string()]),
    );
    query.parameters.0.insert(
        "deleted_at".to_string(),
        (Similarity::Equals, vec!["null".to_string()]),
    );

    let values = query.to_values().unwrap();

    assert_eq!(values.len(), 6);
    assert_eq!(values[0], SqlValue::Text("%john%".to_string()));
    assert_eq!(values[1], SqlValue::Integer(25));
    assert_eq!(values[2], SqlValue::Real(100.5));
    assert_eq!(values[3], SqlValue::Null);
    assert_eq!(values[4], SqlValue::Integer(50)); // limit
    assert_eq!(values[5], SqlValue::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_complex_numeric_types() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "integers".to_string(),
        (
            Similarity::Equals,
            vec!["123".to_string(), "-456".to_string(), "0".to_string()],
        ),
    );
    query.parameters.0.insert(
        "reals".to_string(),
        (
            Similarity::Equals,
            vec![
                "123.45".to_string(),
                "-456.78".to_string(),
                "0.0".to_string(),
            ],
        ),
    );
    query.parameters.0.insert(
        "mixed".to_string(),
        (
            Similarity::Equals,
            vec!["123".to_string(), "123.45".to_string(), "text".to_string()],
        ),
    );

    let values = query.to_values().unwrap();

    assert_eq!(values.len(), 11);
    // integers
    assert_eq!(values[0], SqlValue::Integer(123));
    assert_eq!(values[1], SqlValue::Integer(-456));
    assert_eq!(values[2], SqlValue::Integer(0));
    // reals
    assert_eq!(values[3], SqlValue::Real(123.45));
    assert_eq!(values[4], SqlValue::Real(-456.78));
    assert_eq!(values[5], SqlValue::Real(0.0));
    // mixed
    assert_eq!(values[6], SqlValue::Integer(123));
    assert_eq!(values[7], SqlValue::Real(123.45));
    assert_eq!(values[8], SqlValue::Text("text".to_string()));
    // limit and offset
    assert_eq!(values[9], SqlValue::Integer(50));
    assert_eq!(values[10], SqlValue::Integer(0));
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_edge_case_strings() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "special_chars".to_string(),
        (
            Similarity::Contains,
            vec!["hello%world".to_string(), "test_underscore".to_string()],
        ),
    );
    query.parameters.0.insert(
        "unicode".to_string(),
        (
            Similarity::StartsWith,
            vec!["测试".to_string(), "héllo".to_string()],
        ),
    );
    query.parameters.0.insert(
        "empty_string".to_string(),
        (Similarity::Equals, vec!["".to_string()]),
    );

    let values = query.to_values().unwrap();

    assert_eq!(values.len(), 6);
    assert_eq!(values[0], SqlValue::Text("%hello%world%".to_string()));
    assert_eq!(values[1], SqlValue::Text("%test_underscore%".to_string()));
    assert_eq!(values[2], SqlValue::Text("测试%".to_string()));
    assert_eq!(values[3], SqlValue::Text("héllo%".to_string()));
    assert_eq!(values[4], SqlValue::Integer(50)); // limit
    assert_eq!(values[5], SqlValue::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_numeric_edge_cases() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "edge_numbers".to_string(),
        (
            Similarity::Equals,
            vec![
                "0".to_string(),
                "-0".to_string(),
                "0.0".to_string(),
                "-0.0".to_string(),
                "1e10".to_string(),
                "1.5e-5".to_string(),
                "inf".to_string(),
                "nan".to_string(),
            ],
        ),
    );

    let values = query.to_values().unwrap();

    assert_eq!(values.len(), 10);
    assert_eq!(values[0], SqlValue::Integer(0));
    assert_eq!(values[1], SqlValue::Integer(0));
    assert_eq!(values[2], SqlValue::Real(0.0));
    assert_eq!(values[3], SqlValue::Real(-0.0));
    assert_eq!(values[4], SqlValue::Real(1e10));
    assert_eq!(values[5], SqlValue::Real(1.5e-5));
    assert_eq!(values[6], SqlValue::Real(f64::INFINITY));
    // NaN doesn't equal NaN, so we need to check it's a Real with NaN
    match values[7] {
        SqlValue::Real(nan) => assert!(nan.is_nan()),
        _ => panic!("Expected Real(NaN), got {:?}", values[7]),
    }
    assert_eq!(values[8], SqlValue::Integer(50)); // limit
    assert_eq!(values[9], SqlValue::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_large_numbers() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "large_int".to_string(),
        (
            Similarity::Equals,
            vec![
                "9223372036854775807".to_string(),  // i64::MAX
                "-9223372036854775808".to_string(), // i64::MIN
            ],
        ),
    );
    query.parameters.0.insert(
        "large_real".to_string(),
        (
            Similarity::Equals,
            vec![
                "1.7976931348623157e308".to_string(),  // f64::MAX
                "-1.7976931348623157e308".to_string(), // f64::MIN
            ],
        ),
    );

    let values = query.to_values().unwrap();

    assert_eq!(values.len(), 6);
    assert_eq!(values[0], SqlValue::Integer(9223372036854775807));
    assert_eq!(values[1], SqlValue::Integer(-9223372036854775808));
    assert_eq!(values[2], SqlValue::Real(1.7976931348623157e308));
    assert_eq!(values[3], SqlValue::Real(-1.7976931348623157e308));
    assert_eq!(values[4], SqlValue::Integer(50)); // limit
    assert_eq!(values[5], SqlValue::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_null_variations() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "null_tests".to_string(),
        (
            Similarity::Equals,
            vec![
                "null".to_string(),
                "NULL".to_string(),
                "Null".to_string(),
                "nUlL".to_string(),
            ],
        ),
    );

    let values = query.to_values().unwrap();

    assert_eq!(values.len(), 6);
    assert_eq!(values[0], SqlValue::Null);
    assert_eq!(values[1], SqlValue::Text("NULL".to_string()));
    assert_eq!(values[2], SqlValue::Text("Null".to_string()));
    assert_eq!(values[3], SqlValue::Text("nUlL".to_string()));
    assert_eq!(values[4], SqlValue::Integer(50)); // limit
    assert_eq!(values[5], SqlValue::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_whitespace_handling() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "whitespace".to_string(),
        (
            Similarity::Equals,
            vec![
                " 123 ".to_string(),
                " 123.45 ".to_string(),
                " hello ".to_string(),
            ],
        ),
    );

    let values = query.to_values().unwrap();

    assert_eq!(values.len(), 5);
    assert_eq!(values[0], SqlValue::Text(" 123 ".to_string()));
    assert_eq!(values[1], SqlValue::Text(" 123.45 ".to_string()));
    assert_eq!(values[2], SqlValue::Text(" hello ".to_string()));
    assert_eq!(values[3], SqlValue::Integer(50)); // limit
    assert_eq!(values[4], SqlValue::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_empty_parameters() {
    let mut query = Query::new();
    query
        .parameters
        .0
        .insert("empty".to_string(), (Similarity::Equals, vec![]));

    let values = query.to_values().unwrap();

    // Should only contain limit and offset
    assert_eq!(values.len(), 2);
    assert_eq!(values[0], SqlValue::Integer(50)); // limit
    assert_eq!(values[1], SqlValue::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_multiple_parameters_same_key() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "name".to_string(),
        (Similarity::Contains, vec!["john".to_string()]),
    );
    query.parameters.0.insert(
        "name".to_string(), // This should overwrite the previous
        (Similarity::StartsWith, vec!["jane".to_string()]),
    );

    let values = query.to_values().unwrap();

    assert_eq!(values.len(), 3);
    assert_eq!(values[0], SqlValue::Text("jane%".to_string()));
    assert_eq!(values[1], SqlValue::Integer(50)); // limit
    assert_eq!(values[2], SqlValue::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_parameter_order_preservation() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "first".to_string(),
        (Similarity::Equals, vec!["1".to_string()]),
    );
    query.parameters.0.insert(
        "second".to_string(),
        (Similarity::Contains, vec!["2".to_string()]),
    );
    query.parameters.0.insert(
        "third".to_string(),
        (Similarity::Between, vec!["3".to_string(), "4".to_string()]),
    );

    let values = query.to_values().unwrap();

    assert_eq!(values.len(), 6);
    assert_eq!(values[0], SqlValue::Integer(1));
    assert_eq!(values[1], SqlValue::Text("%2%".to_string()));
    assert_eq!(values[2], SqlValue::Integer(3));
    assert_eq!(values[3], SqlValue::Integer(4));
    assert_eq!(values[4], SqlValue::Integer(50)); // limit
    assert_eq!(values[5], SqlValue::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_with_custom_limit_offset_complex() {
    let mut query = Query::new();
    query.limit = 100;
    query.offset = 25;

    query.parameters.0.insert(
        "name".to_string(),
        (Similarity::Contains, vec!["john".to_string()]),
    );
    query.parameters.0.insert(
        "age".to_string(),
        (
            Similarity::Between,
            vec!["20".to_string(), "30".to_string()],
        ),
    );

    let values = query.to_values().unwrap();

    assert_eq!(values.len(), 5);
    assert_eq!(values[0], SqlValue::Text("%john%".to_string()));
    assert_eq!(values[1], SqlValue::Integer(20));
    assert_eq!(values[2], SqlValue::Integer(30));
    assert_eq!(values[3], SqlValue::Integer(100)); // custom limit
    assert_eq!(values[4], SqlValue::Integer(25)); // custom offset
}

#[cfg(feature = "sql")]
#[test]
fn test_sql_value_enum_variants() {
    // Test all SQLValue enum variants
    let null = SqlValue::Null;
    let integer = SqlValue::Integer(42);
    let real = SqlValue::Real(3.14);
    let text = SqlValue::Text("hello".to_string());
    let blob = SqlValue::Blob(vec![1, 2, 3, 4]);

    assert_eq!(null, SqlValue::Null);
    assert_eq!(integer, SqlValue::Integer(42));
    assert_eq!(real, SqlValue::Real(3.14));
    assert_eq!(text, SqlValue::Text("hello".to_string()));
    assert_eq!(blob, SqlValue::Blob(vec![1, 2, 3, 4]));
}

#[cfg(feature = "sql")]
#[test]
fn test_sql_value_enum_derived_traits() {
    // Test Clone
    let original = SqlValue::Text("test".to_string());
    let cloned = original.clone();
    assert_eq!(original, cloned);

    // Test Debug
    let debug_str = format!("{:?}", SqlValue::Integer(42));
    assert!(debug_str.contains("Integer"));
    assert!(debug_str.contains("42"));

    // Test PartialEq
    assert_eq!(SqlValue::Null, SqlValue::Null);
    assert_eq!(SqlValue::Integer(42), SqlValue::Integer(42));
    assert_eq!(SqlValue::Real(3.14), SqlValue::Real(3.14));
    assert_eq!(
        SqlValue::Text("hello".to_string()),
        SqlValue::Text("hello".to_string())
    );
    assert_eq!(SqlValue::Blob(vec![1, 2, 3]), SqlValue::Blob(vec![1, 2, 3]));

    // Test inequality
    assert_ne!(SqlValue::Null, SqlValue::Integer(0));
    assert_ne!(SqlValue::Integer(42), SqlValue::Integer(43));
    assert_ne!(SqlValue::Real(3.14), SqlValue::Real(3.15));
    assert_ne!(
        SqlValue::Text("hello".to_string()),
        SqlValue::Text("world".to_string())
    );
    assert_ne!(SqlValue::Blob(vec![1, 2, 3]), SqlValue::Blob(vec![1, 2, 4]));
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_parameter_values() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "name".to_string(),
        (Similarity::Contains, vec!["john".to_string()]),
    );
    query.parameters.0.insert(
        "age".to_string(),
        (Similarity::Equals, vec!["25".to_string()]),
    );

    let values = query.to_parameter_values().unwrap();

    assert_eq!(values.len(), 2);
    assert_eq!(values[0], SqlValue::Text("%john%".to_string()));
    assert_eq!(values[1], SqlValue::Integer(25));
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_parameter_values_empty() {
    let query = Query::new();
    let values = query.to_parameter_values().unwrap();

    assert_eq!(values.len(), 0);
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_pagination_values() {
    let mut query = Query::new();
    query.limit = 100;
    query.offset = 25;

    let values = query.to_pagination_values();

    assert_eq!(values.len(), 2);
    assert_eq!(values[0], SqlValue::Integer(100));
    assert_eq!(values[1], SqlValue::Integer(25));
}

#[cfg(feature = "sql")]
#[test]
fn test_query_total_parameters() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "name".to_string(),
        (
            Similarity::Contains,
            vec!["john".to_string(), "jane".to_string()],
        ),
    );
    query.parameters.0.insert(
        "age".to_string(),
        (Similarity::Equals, vec!["25".to_string()]),
    );

    let count = query.total_parameters();

    // 2 name values + 1 age value + 2 pagination = 5 total
    assert_eq!(count, 5);
}

#[cfg(feature = "sql")]
#[test]
fn test_query_total_parameters_empty() {
    let query = Query::new();
    let count = query.total_parameters();

    // Only pagination values
    assert_eq!(count, 2);
}

#[cfg(feature = "sql")]
#[test]
fn test_query_total_parameters_with_empty_parameters() {
    let mut query = Query::new();
    query
        .parameters
        .0
        .insert("empty".to_string(), (Similarity::Equals, vec![]));

    let count = query.total_parameters();

    // No parameter values + 2 pagination = 2 total
    assert_eq!(count, 2);
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_ignores_empty_values() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "name".to_string(),
        (
            Similarity::Contains,
            vec![
                "john".to_string(),
                "".to_string(),        // empty string
                "jane".to_string(),
                "   ".to_string(),     // whitespace only
                "bob".to_string(),
            ],
        ),
    );
    query.parameters.0.insert(
        "age".to_string(),
        (
            Similarity::Equals,
            vec![
                "25".to_string(),
                "".to_string(),        // empty string
                "30".to_string(),
            ],
        ),
    );

    let values = query.to_values().unwrap();

    // Should only contain non-empty values: john, jane, bob, 25, 30 + limit + offset = 7
    assert_eq!(values.len(), 7);
    assert_eq!(values[0], SqlValue::Text("%john%".to_string()));
    assert_eq!(values[1], SqlValue::Text("%jane%".to_string()));
    assert_eq!(values[2], SqlValue::Text("%bob%".to_string()));
    assert_eq!(values[3], SqlValue::Integer(25));
    assert_eq!(values[4], SqlValue::Integer(30));
    assert_eq!(values[5], SqlValue::Integer(50)); // limit
    assert_eq!(values[6], SqlValue::Integer(0));  // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_parameter_values_ignores_empty_values() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "name".to_string(),
        (
            Similarity::Contains,
            vec![
                "john".to_string(),
                "".to_string(),        // empty string
                "jane".to_string(),
                "   ".to_string(),     // whitespace only
            ],
        ),
    );

    let values = query.to_parameter_values().unwrap();

    // Should only contain non-empty values: john, jane
    assert_eq!(values.len(), 2);
    assert_eq!(values[0], SqlValue::Text("%john%".to_string()));
    assert_eq!(values[1], SqlValue::Text("%jane%".to_string()));
}

#[cfg(feature = "sql")]
#[test]
fn test_query_total_parameters_ignores_empty_values() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "name".to_string(),
        (
            Similarity::Contains,
            vec![
                "john".to_string(),
                "".to_string(),        // empty string
                "jane".to_string(),
                "   ".to_string(),     // whitespace only
            ],
        ),
    );
    query.parameters.0.insert(
        "age".to_string(),
        (
            Similarity::Equals,
            vec![
                "25".to_string(),
                "".to_string(),        // empty string
            ],
        ),
    );

    let count = query.total_parameters();

    // 2 non-empty name values + 1 non-empty age value + 2 pagination = 5 total
    assert_eq!(count, 5);
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_all_empty_values() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "name".to_string(),
        (
            Similarity::Contains,
            vec![
                "".to_string(),        // empty string
                "   ".to_string(),     // whitespace only
                "\t".to_string(),      // tab only
                "\n".to_string(),      // newline only
            ],
        ),
    );

    let values = query.to_values().unwrap();

    // Should only contain limit and offset (no parameter values)
    assert_eq!(values.len(), 2);
    assert_eq!(values[0], SqlValue::Integer(50)); // limit
    assert_eq!(values[1], SqlValue::Integer(0));  // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_mixed_empty_and_null() {
    let mut query = Query::new();
    query.parameters.0.insert(
        "status".to_string(),
        (
            Similarity::Equals,
            vec![
                "".to_string(),        // empty string (should be ignored)
                "null".to_string(),    // null string (should be converted to SQLValue::Null)
                "   ".to_string(),     // whitespace only (should be ignored)
                "active".to_string(),  // normal value
            ],
        ),
    );

    let values = query.to_values().unwrap();

    // Should contain: null, active + limit + offset = 4 total
    assert_eq!(values.len(), 4);
    assert_eq!(values[0], SqlValue::Null);
    assert_eq!(values[1], SqlValue::Text("active".to_string()));
    assert_eq!(values[2], SqlValue::Integer(50)); // limit
    assert_eq!(values[3], SqlValue::Integer(0));  // offset
}
