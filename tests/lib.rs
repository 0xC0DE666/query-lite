use query_lite::error::Error;
use query_lite::*;
use std::str::FromStr;

// ============================================================================
// SORT ORDER TESTS
// ============================================================================

#[test]
fn test_sort_order_variants() {
    assert_eq!(SortDirection::Ascending, SortDirection::Ascending);
    assert_eq!(SortDirection::Descending, SortDirection::Descending);
    assert_ne!(SortDirection::Ascending, SortDirection::Descending);
}

#[test]
fn test_sort_order_constants() {
    assert_eq!(SortDirection::ASCENDING, "asc");
    assert_eq!(SortDirection::DESCENDING, "desc");
}

#[test]
fn test_sort_order_default() {
    assert_eq!(SortDirection::default(), SortDirection::Ascending);
}

#[test]
fn test_sort_order_from_str_valid() {
    assert_eq!(
        SortDirection::from_str("asc").unwrap(),
        SortDirection::Ascending
    );
    assert_eq!(
        SortDirection::from_str("desc").unwrap(),
        SortDirection::Descending
    );
}

#[test]
fn test_sort_order_from_str_invalid() {
    assert!(SortDirection::from_str("invalid").is_err());
    assert!(SortDirection::from_str("").is_err());
    assert!(SortDirection::from_str("ASC").is_err());
    assert!(SortDirection::from_str("DESC").is_err());
    assert!(SortDirection::from_str("ascending").is_err());
    assert!(SortDirection::from_str("descending").is_err());
}

#[test]
fn test_sort_order_display() {
    assert_eq!(format!("{}", SortDirection::Ascending), "asc");
    assert_eq!(format!("{}", SortDirection::Descending), "desc");
}

// ============================================================================
// ORDER FIELD TESTS
// ============================================================================

#[test]
fn test_parse_order_field_asc() {
    let order_field = "name:asc".parse::<OrderField>().unwrap();
    assert_eq!(order_field.name(), "name");
    assert_eq!(*order_field.sort_direction(), SortDirection::Ascending);
}

#[test]
fn test_parse_order_field_desc() {
    let order_field = "date_created:desc".parse::<OrderField>().unwrap();
    assert_eq!(order_field.name(), "date_created");
    assert_eq!(*order_field.sort_direction(), SortDirection::Descending);
}

#[test]
fn test_parse_order_field_with_whitespace() {
    let order_field = "  name  :  asc  ".parse::<OrderField>().unwrap();
    assert_eq!(order_field.name(), "name");
    assert_eq!(*order_field.sort_direction(), SortDirection::Ascending);
}

#[test]
fn test_parse_order_field_with_special_characters() {
    let order_field = "user_name:desc".parse::<OrderField>().unwrap();
    assert_eq!(order_field.name(), "user_name");
    assert_eq!(*order_field.sort_direction(), SortDirection::Descending);
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
fn test_order_field_display() {
    let order_field = "name:asc".parse::<OrderField>().unwrap();
    assert_eq!(format!("{}", order_field), "name:asc");

    let order_field = "date_created:desc".parse::<OrderField>().unwrap();
    assert_eq!(format!("{}", order_field), "date_created:desc");
}

#[test]
fn test_order_display() {
    let mut order = Order::new();
    order.ascending("name".to_string());
    order.descending("date_created".to_string());

    let display_str = format!("{}", order);
    assert!(display_str.contains("name:asc"));
    assert!(display_str.contains("date_created:desc"));
    assert!(display_str.contains(","));
}

#[test]
fn test_order_display_empty() {
    let order = Order::new();
    assert_eq!(format!("{}", order), "");
}

#[test]
fn test_order_display_filters_empty_names() {
    let mut order = Order::new();
    order.ascending("name".to_string());
    order.ascending("".to_string());
    order.descending("date_created".to_string());

    let display_str = format!("{}", order);
    assert!(display_str.contains("name:asc"));
    assert!(display_str.contains("date_created:desc"));
    // Should not contain empty name
    let parts: Vec<&str> = display_str.split(',').collect();
    assert_eq!(parts.len(), 2);
}

#[test]
fn test_order_display_multiple_fields() {
    let mut order = Order::new();
    order.ascending("name".to_string());
    order.descending("date_created".to_string());
    order.ascending("email".to_string());

    let display_str = format!("{}", order);
    let parts: Vec<&str> = display_str.split(',').collect();
    assert_eq!(parts.len(), 3);
    assert!(display_str.contains("name:asc"));
    assert!(display_str.contains("date_created:desc"));
    assert!(display_str.contains("email:asc"));
}

// ============================================================================
// SORT FIELDS HELPER FUNCTIONS TESTS
// ============================================================================

#[test]
fn test_order_helper_functions_asc() {
    let mut fields = Order::new();
    fields.ascending("name".to_string());

    assert_eq!(fields.inner().len(), 1);
    assert_eq!(fields.inner().get("name"), Some(&SortDirection::Ascending));
}

#[test]
fn test_order_helper_functions_desc() {
    let mut fields = Order::new();
    fields.descending("date_created".to_string());

    assert_eq!(fields.inner().len(), 1);
    assert_eq!(
        fields.inner().get("date_created"),
        Some(&SortDirection::Descending)
    );
}

#[test]
fn test_order_helper_functions_fluent_api() {
    let mut fields = Order::new();
    fields
        .ascending("name".to_string())
        .descending("date_created".to_string())
        .ascending("email".to_string());

    assert_eq!(fields.inner().len(), 3);
    assert_eq!(fields.inner().get("name"), Some(&SortDirection::Ascending));
    assert_eq!(
        fields.inner().get("date_created"),
        Some(&SortDirection::Descending)
    );
    assert_eq!(fields.inner().get("email"), Some(&SortDirection::Ascending));
}

#[test]
fn test_order_helper_functions_overwrite() {
    let mut fields = Order::new();
    fields.ascending("name".to_string());
    fields.descending("name".to_string()); // Should overwrite

    assert_eq!(fields.inner().len(), 1);
    assert_eq!(fields.inner().get("name"), Some(&SortDirection::Descending));
}

#[test]
fn test_order_helper_functions_empty_name() {
    let mut fields = Order::new();
    fields.ascending("".to_string());

    assert_eq!(fields.inner().len(), 1);
    assert_eq!(fields.inner().get(""), Some(&SortDirection::Ascending));
}

#[test]
fn test_order_helper_functions_whitespace_name() {
    let mut fields = Order::new();
    fields.ascending("  name  ".to_string());

    assert_eq!(fields.inner().len(), 1);
    assert_eq!(
        fields.inner().get("  name  "),
        Some(&SortDirection::Ascending)
    );
}

#[test]
fn test_order_helper_functions_special_characters() {
    let mut fields = Order::new();
    fields.ascending("user_name_123".to_string());

    assert_eq!(fields.inner().len(), 1);
    assert_eq!(
        fields.inner().get("user_name_123"),
        Some(&SortDirection::Ascending)
    );
}

#[test]
fn test_order_helper_functions_unicode_name() {
    let mut fields = Order::new();
    fields.ascending("用户_姓名".to_string());

    assert_eq!(fields.inner().len(), 1);
    assert_eq!(
        fields.inner().get("用户_姓名"),
        Some(&SortDirection::Ascending)
    );
}

#[test]
fn test_order_keep() {
    let mut fields = Order::new();
    fields.ascending("name".to_string());
    fields.descending("date_created".to_string());
    fields.ascending("email".to_string());

    let filtered = fields.keep(vec!["name".to_string(), "email".to_string()]);

    assert_eq!(filtered.inner().len(), 2);
    assert_eq!(
        filtered.inner().get("name"),
        Some(&SortDirection::Ascending)
    );
    assert_eq!(filtered.inner().get("date_created"), None);
    assert_eq!(
        filtered.inner().get("email"),
        Some(&SortDirection::Ascending)
    );
}

#[test]
fn test_order_keep_nonexistent_keys() {
    let mut fields = Order::new();
    fields.ascending("name".to_string());

    let filtered = fields.keep(vec!["nonexistent".to_string()]);
    assert_eq!(filtered.inner().len(), 0);
}

#[test]
fn test_order_keep_empty_keys() {
    let mut fields = Order::new();
    fields.ascending("name".to_string());

    let filtered = fields.keep(vec![]);
    assert_eq!(filtered.inner().len(), 0);
}

#[test]
fn test_order_remove() {
    let mut fields = Order::new();
    fields.ascending("name".to_string());
    fields.descending("date_created".to_string());
    fields.ascending("email".to_string());

    let filtered = fields.remove(vec!["name".to_string(), "email".to_string()]);

    assert_eq!(filtered.inner().len(), 1);
    assert_eq!(filtered.inner().get("name"), None);
    assert_eq!(
        filtered.inner().get("date_created"),
        Some(&SortDirection::Descending)
    );
    assert_eq!(filtered.inner().get("email"), None);
}

#[test]
fn test_order_remove_nonexistent_keys() {
    let mut fields = Order::new();
    fields.ascending("name".to_string());

    let filtered = fields.remove(vec!["nonexistent".to_string()]);
    assert_eq!(filtered.inner().len(), 1);
    assert_eq!(
        filtered.inner().get("name"),
        Some(&SortDirection::Ascending)
    );
}

#[test]
fn test_order_remove_empty_keys() {
    let mut fields = Order::new();
    fields.ascending("name".to_string());

    let filtered = fields.remove(vec![]);
    assert_eq!(filtered.inner().len(), 1);
    assert_eq!(
        filtered.inner().get("name"),
        Some(&SortDirection::Ascending)
    );
}

// ============================================================================
// SORT FIELDS TESTS
// ============================================================================

#[test]
fn test_order_new() {
    let fields = Order::new();
    assert_eq!(fields.inner().len(), 0);
}

#[test]
fn test_order_default() {
    let fields = Order::default();
    assert_eq!(fields.inner().len(), 0);
}

#[test]
fn test_order_from_str_empty() {
    let fields = Order::from_str("").unwrap();
    assert_eq!(fields.inner().len(), 0);

    let fields = Order::from_str("   ").unwrap();
    assert_eq!(fields.inner().len(), 0);
}

#[test]
fn test_order_from_str_single() {
    let fields = Order::from_str("name:asc").unwrap();
    assert_eq!(fields.inner().len(), 1);
    assert_eq!(fields.inner().get("name"), Some(&SortDirection::Ascending));
}

#[test]
fn test_order_from_str_multiple() {
    let fields = Order::from_str("date_created:desc,name:asc,surname:asc").unwrap();
    assert_eq!(fields.inner().len(), 3);

    assert_eq!(
        fields.inner().get("date_created"),
        Some(&SortDirection::Descending)
    );
    assert_eq!(fields.inner().get("name"), Some(&SortDirection::Ascending));
    assert_eq!(
        fields.inner().get("surname"),
        Some(&SortDirection::Ascending)
    );
}

#[test]
fn test_order_from_str_with_whitespace() {
    let fields = Order::from_str("  date_created:desc  ,  name:asc  ,  surname:asc  ").unwrap();
    assert_eq!(fields.inner().len(), 3);
    assert_eq!(
        fields.inner().get("date_created"),
        Some(&SortDirection::Descending)
    );
    assert_eq!(fields.inner().get("name"), Some(&SortDirection::Ascending));
    assert_eq!(
        fields.inner().get("surname"),
        Some(&SortDirection::Ascending)
    );
}

#[test]
fn test_order_from_str_with_empty_fields() {
    let fields = Order::from_str("name:asc,,surname:asc").unwrap();
    assert_eq!(fields.inner().len(), 2);
    assert_eq!(fields.inner().get("name"), Some(&SortDirection::Ascending));
    assert_eq!(
        fields.inner().get("surname"),
        Some(&SortDirection::Ascending)
    );
}

#[test]
fn test_order_from_str_invalid() {
    // Invalid field format
    assert!(Order::from_str("name").is_err());
    assert!(Order::from_str("name:invalid").is_err());
    assert!(Order::from_str("name:asc,surname").is_err());
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
fn test_similarity_display() {
    assert_eq!(format!("{}", Similarity::Equals), "equals");
    assert_eq!(format!("{}", Similarity::Contains), "contains");
    assert_eq!(format!("{}", Similarity::StartsWith), "starts-with");
    assert_eq!(format!("{}", Similarity::EndsWith), "ends-with");
    assert_eq!(format!("{}", Similarity::Between), "between");
    assert_eq!(format!("{}", Similarity::Lesser), "lesser");
    assert_eq!(format!("{}", Similarity::LesserOrEqual), "lesser-or-equal");
    assert_eq!(format!("{}", Similarity::Greater), "greater");
    assert_eq!(
        format!("{}", Similarity::GreaterOrEqual),
        "greater-or-equal"
    );
}

// ============================================================================
// PARAMETER TESTS
// ============================================================================

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
    let param = "equals: black , steel , wood "
        .parse::<Parameter>()
        .unwrap();
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

#[test]
fn test_error_invalid_parameter() {
    let error = "invalid".parse::<Parameter>().unwrap_err();
    match error {
        Error::InvalidParameter(msg) => assert_eq!(msg, "invalid"),
        _ => panic!("Expected InvalidParameter error"),
    }
}

#[test]
fn test_parameter_display() {
    let param = Parameter::init(Similarity::Contains, vec!["damian".to_string()]);
    assert_eq!(format!("{}", param), "contains:damian");

    let param = Parameter::init(
        Similarity::Equals,
        vec!["black".to_string(), "steel".to_string(), "wood".to_string()],
    );
    assert_eq!(format!("{}", param), "equals:black,steel,wood");

    let param = Parameter::init(
        Similarity::Between,
        vec!["20".to_string(), "30".to_string()],
    );
    assert_eq!(format!("{}", param), "between:20,30");
}

#[test]
fn test_parameter_display_with_url_encoding() {
    let param = Parameter::init(
        Similarity::Contains,
        vec!["hello world".to_string(), "test&value".to_string()],
    );
    let display_str = format!("{}", param);
    // Should contain URL-encoded values
    assert!(display_str.contains("contains:"));
    // The exact encoding depends on url_encode implementation
}

#[test]
fn test_parameters_display() {
    let mut params = Parameters::new();
    params.contains("name".to_string(), vec!["damian".to_string()]);
    params.equals(
        "surname".to_string(),
        vec!["black".to_string(), "steel".to_string()],
    );

    let display_str = format!("{}", params);
    assert!(display_str.contains("name=contains:damian"));
    assert!(display_str.contains("surname=equals:black,steel"));
    assert!(display_str.contains("&"));
}

#[test]
fn test_parameters_display_empty() {
    let params = Parameters::new();
    assert_eq!(format!("{}", params), "");
}

#[test]
fn test_parameters_display_filters_empty_values() {
    let mut params = Parameters::new();
    params.contains("name".to_string(), vec!["damian".to_string()]);
    params.equals("empty".to_string(), vec![]);

    let display_str = format!("{}", params);
    assert!(display_str.contains("name=contains:damian"));
    assert!(!display_str.contains("empty"));
}

#[test]
fn test_parameters_display_multiple_parameters() {
    let mut params = Parameters::new();
    params.contains("name".to_string(), vec!["john".to_string()]);
    params.equals("status".to_string(), vec!["active".to_string()]);
    params.between("age".to_string(), vec!["20".to_string(), "30".to_string()]);

    let display_str = format!("{}", params);
    // Should contain all three parameters joined by &
    let parts: Vec<&str> = display_str.split('&').collect();
    assert_eq!(parts.len(), 3);
    assert!(display_str.contains("name=contains:john"));
    assert!(display_str.contains("status=equals:active"));
    assert!(display_str.contains("age=between:20,30"));
}

// ============================================================================
// PARAMETERS BUILDER TESTS
// ============================================================================

#[test]
fn test_parameters_builder_equals() {
    let mut params = Parameters::new();
    let values = vec!["value1".to_string(), "value2".to_string()];
    params.equals("name".to_string(), values.clone());

    assert_eq!(params.inner().len(), 1);
    assert!(params.inner().contains_key("name"));
    let param = &params.inner()["name"];
    assert_eq!(*param.similarity(), Similarity::Equals);
    assert_eq!(*param.values(), values);
}

#[test]
fn test_parameters_builder_contains() {
    let mut params = Parameters::new();
    let values = vec!["value1".to_string(), "value2".to_string()];
    params.contains("name".to_string(), values.clone());

    assert_eq!(params.inner().len(), 1);
    assert!(params.inner().contains_key("name"));
    let param = &params.inner()["name"];
    assert_eq!(*param.similarity(), Similarity::Contains);
    assert_eq!(*param.values(), values);
}

#[test]
fn test_parameters_builder_starts_with() {
    let mut params = Parameters::new();
    let values = vec!["value1".to_string(), "value2".to_string()];
    params.starts_with("name".to_string(), values.clone());

    assert_eq!(params.inner().len(), 1);
    assert!(params.inner().contains_key("name"));
    let param = &params.inner()["name"];
    assert_eq!(*param.similarity(), Similarity::StartsWith);
    assert_eq!(*param.values(), values);
}

#[test]
fn test_parameters_builder_ends_with() {
    let mut params = Parameters::new();
    let values = vec!["value1".to_string(), "value2".to_string()];
    params.ends_with("name".to_string(), values.clone());

    assert_eq!(params.inner().len(), 1);
    assert!(params.inner().contains_key("name"));
    let param = &params.inner()["name"];
    assert_eq!(*param.similarity(), Similarity::EndsWith);
    assert_eq!(*param.values(), values);
}

#[test]
fn test_parameters_builder_between() {
    let mut params = Parameters::new();
    let values = vec!["20".to_string(), "30".to_string()];
    params.between("age".to_string(), values.clone());

    assert_eq!(params.inner().len(), 1);
    assert!(params.inner().contains_key("age"));
    let param = &params.inner()["age"];
    assert_eq!(*param.similarity(), Similarity::Between);
    assert_eq!(*param.values(), values);
}

#[test]
fn test_parameters_builder_lesser() {
    let mut params = Parameters::new();
    let values = vec!["100".to_string()];
    params.lesser("price".to_string(), values.clone());

    assert_eq!(params.inner().len(), 1);
    assert!(params.inner().contains_key("price"));
    let param = &params.inner()["price"];
    assert_eq!(*param.similarity(), Similarity::Lesser);
    assert_eq!(*param.values(), values);
}

#[test]
fn test_parameters_builder_lesser_or_equal() {
    let mut params = Parameters::new();
    let values = vec!["100".to_string()];
    params.lesser_or_equal("price".to_string(), values.clone());

    assert_eq!(params.inner().len(), 1);
    assert!(params.inner().contains_key("price"));
    let param = &params.inner()["price"];
    assert_eq!(*param.similarity(), Similarity::LesserOrEqual);
    assert_eq!(*param.values(), values);
}

#[test]
fn test_parameters_builder_greater() {
    let mut params = Parameters::new();
    let values = vec!["50".to_string()];
    params.greater("price".to_string(), values.clone());

    assert_eq!(params.inner().len(), 1);
    assert!(params.inner().contains_key("price"));
    let param = &params.inner()["price"];
    assert_eq!(*param.similarity(), Similarity::Greater);
    assert_eq!(*param.values(), values);
}

#[test]
fn test_parameters_builder_greater_or_equal() {
    let mut params = Parameters::new();
    let values = vec!["50".to_string()];
    params.greater_or_equal("price".to_string(), values.clone());

    assert_eq!(params.inner().len(), 1);
    assert!(params.inner().contains_key("price"));
    let param = &params.inner()["price"];
    assert_eq!(*param.similarity(), Similarity::GreaterOrEqual);
    assert_eq!(*param.values(), values);
}

#[test]
fn test_parameters_builder_fluent_api() {
    let mut params = Parameters::new();
    params
        .equals("name".to_string(), vec!["damian".to_string()])
        .contains("surname".to_string(), vec!["black".to_string()])
        .between("age".to_string(), vec!["20".to_string(), "30".to_string()]);

    assert_eq!(params.inner().len(), 3);

    let param = &params.inner()["name"];
    assert_eq!(*param.similarity(), Similarity::Equals);
    assert_eq!(*param.values(), vec!["damian"]);

    let param = &params.inner()["surname"];
    assert_eq!(*param.similarity(), Similarity::Contains);
    assert_eq!(*param.values(), vec!["black"]);

    let param = &params.inner()["age"];
    assert_eq!(*param.similarity(), Similarity::Between);
    assert_eq!(*param.values(), vec!["20", "30"]);
}

#[test]
fn test_parameters_builder_overwrite() {
    let mut params = Parameters::new();
    params.equals("name".to_string(), vec!["damian".to_string()]);
    params.contains("name".to_string(), vec!["john".to_string()]); // Should overwrite

    assert_eq!(params.inner().len(), 1);
    let param = &params.inner()["name"];
    assert_eq!(*param.similarity(), Similarity::Contains);
    assert_eq!(*param.values(), vec!["john"]);
}

#[test]
fn test_parameters_builder_empty_values() {
    let mut params = Parameters::new();
    params.equals("name".to_string(), vec![]);

    assert_eq!(params.inner().len(), 1);
    let param = &params.inner()["name"];
    assert_eq!(*param.similarity(), Similarity::Equals);
    assert_eq!(*param.values(), vec![] as Vec<String>);
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
    assert_eq!(params.inner().len(), 0);
}

#[test]
fn test_parameters_default() {
    let params = Parameters::default();
    assert_eq!(params.inner().len(), 0);
}

#[test]
fn test_parameters_from_str_empty() {
    let params = Parameters::from_str("").unwrap();
    assert_eq!(params.inner().len(), 0);

    let params = Parameters::from_str("   ").unwrap();
    assert_eq!(params.inner().len(), 0);
}

#[test]
fn test_parameters_from_str_single() {
    let params = Parameters::from_str("name=contains:damian").unwrap();
    assert_eq!(params.inner().len(), 1);
    assert!(params.inner().contains_key("name"));
    let param = &params.inner()["name"];
    assert_eq!(*param.similarity(), Similarity::Contains);
    assert_eq!(*param.values(), vec!["damian"]);
}

#[test]
fn test_parameters_from_str_multiple() {
    let params =
        Parameters::from_str("name=contains:damian&surname=equals:black,steel,wood").unwrap();
    assert_eq!(params.inner().len(), 2);

    assert!(params.inner().contains_key("name"));
    let param = &params.inner()["name"];
    assert_eq!(*param.similarity(), Similarity::Contains);
    assert_eq!(*param.values(), vec!["damian"]);

    assert!(params.inner().contains_key("surname"));
    let param = &params.inner()["surname"];
    assert_eq!(*param.similarity(), Similarity::Equals);
    assert_eq!(*param.values(), vec!["black", "steel", "wood"]);
}

#[test]
fn test_parameters_from_str_with_whitespace() {
    let params =
        Parameters::from_str("  name  =  contains:damian  &  surname  =  equals:black  ").unwrap();
    assert_eq!(params.inner().len(), 2);
    assert!(params.inner().contains_key("name"));
    assert!(params.inner().contains_key("surname"));
}

#[test]
fn test_parameters_from_str_excludes_special_params() {
    let params =
        Parameters::from_str("name=contains:damian&order=date_created:desc&limit=40&offset=0")
            .unwrap();
    assert_eq!(params.inner().len(), 1);
    assert!(params.inner().contains_key("name"));
    assert!(!params.inner().contains_key("order"));
    assert!(!params.inner().contains_key("limit"));
    assert!(!params.inner().contains_key("offset"));
}

#[test]
fn test_parameters_from_str_empty_params() {
    let params = Parameters::from_str("name=contains:damian&&surname=equals:black&").unwrap();
    assert_eq!(params.inner().len(), 2);
    assert!(params.inner().contains_key("name"));
    assert!(params.inner().contains_key("surname"));
}

#[test]
fn test_parameters_from_str_invalid_key() {
    let params = Parameters::from_str("=contains:damian").unwrap();
    assert_eq!(params.inner().len(), 0);
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
    assert_eq!(params.inner().len(), 3);

    assert!(params.inner().contains_key("age"));
    let param = &params.inner()["age"];
    assert_eq!(*param.similarity(), Similarity::Between);
    assert_eq!(*param.values(), vec!["20", "30"]);

    assert!(params.inner().contains_key("price"));
    let param = &params.inner()["price"];
    assert_eq!(*param.similarity(), Similarity::Greater);
    assert_eq!(*param.values(), vec!["100"]);

    assert!(params.inner().contains_key("score"));
    let param = &params.inner()["score"];
    assert_eq!(*param.similarity(), Similarity::LesserOrEqual);
    assert_eq!(*param.values(), vec!["85"]);
}

#[test]
fn test_parameters_from_str_mixed_similarity_types() {
    let params = Parameters::from_str(
        "name=contains:damian&age=between:25,35&price=greater-or-equal:50&status=equals:active",
    )
    .unwrap();
    assert_eq!(params.inner().len(), 4);

    assert!(params.inner().contains_key("name"));
    let param = &params.inner()["name"];
    assert_eq!(*param.similarity(), Similarity::Contains);
    assert_eq!(*param.values(), vec!["damian"]);

    assert!(params.inner().contains_key("age"));
    let param = &params.inner()["age"];
    assert_eq!(*param.similarity(), Similarity::Between);
    assert_eq!(*param.values(), vec!["25", "35"]);

    assert!(params.inner().contains_key("price"));
    let param = &params.inner()["price"];
    assert_eq!(*param.similarity(), Similarity::GreaterOrEqual);
    assert_eq!(*param.values(), vec!["50"]);

    assert!(params.inner().contains_key("status"));
    let param = &params.inner()["status"];
    assert_eq!(*param.similarity(), Similarity::Equals);
    assert_eq!(*param.values(), vec!["active"]);
}

#[test]
fn test_parameters_from_str_numeric_with_whitespace() {
    let params =
        Parameters::from_str("  age  =  between:20,30  &  price  =  lesser:100  ").unwrap();
    assert_eq!(params.inner().len(), 2);
    assert!(params.inner().contains_key("age"));
    assert!(params.inner().contains_key("price"));
}

// ============================================================================
// QUERY TESTS
// ============================================================================

#[test]
fn test_query_new() {
    let query = Query::new();
    assert_eq!(query.parameters.inner().len(), 0);
    assert_eq!(query.order.inner().len(), 0);
    assert_eq!(query.limit, Parameters::DEFAULT_LIMIT);
    assert_eq!(query.offset, Parameters::DEFAULT_OFFSET);
}

#[test]
fn test_query_init() {
    let params = Parameters::new();
    let order = Order::new();
    let query = Query::init(params, order, 100, 10);
    assert_eq!(query.parameters.inner().len(), 0);
    assert_eq!(query.order.inner().len(), 0);
    assert_eq!(query.limit, 100);
    assert_eq!(query.offset, 10);
}


#[test]
fn test_parameters_keep() {
    let mut params = Parameters::new();
    params.inner_mut().insert(
        "name".to_string(),
        Parameter::init(Similarity::Contains, vec!["value1".to_string()]),
    );
    params.inner_mut().insert(
        "surname".to_string(),
        Parameter::init(Similarity::Equals, vec!["value2".to_string()]),
    );
    params.inner_mut().insert(
        "email".to_string(),
        Parameter::init(Similarity::StartsWith, vec!["value3".to_string()]),
    );

    let filtered = params.keep(vec!["name".to_string(), "email".to_string()]);

    assert_eq!(filtered.inner().len(), 2);
    assert!(filtered.inner().contains_key("name"));
    assert!(!filtered.inner().contains_key("surname"));
    assert!(filtered.inner().contains_key("email"));
}

#[test]
fn test_parameters_keep_nonexistent_keys() {
    let mut params = Parameters::new();
    params.inner_mut().insert(
        "name".to_string(),
        Parameter::init(Similarity::Contains, vec!["value1".to_string()]),
    );

    let filtered = params.keep(vec!["nonexistent".to_string()]);
    assert_eq!(filtered.inner().len(), 0);
}

#[test]
fn test_parameters_keep_empty_keys() {
    let mut params = Parameters::new();
    params.inner_mut().insert(
        "name".to_string(),
        Parameter::init(Similarity::Contains, vec!["value1".to_string()]),
    );

    let filtered = params.keep(vec![]);
    assert_eq!(filtered.inner().len(), 0);
}

#[test]
fn test_parameters_remove() {
    let mut params = Parameters::new();
    params.inner_mut().insert(
        "name".to_string(),
        Parameter::init(Similarity::Contains, vec!["value1".to_string()]),
    );
    params.inner_mut().insert(
        "surname".to_string(),
        Parameter::init(Similarity::Equals, vec!["value2".to_string()]),
    );
    params.inner_mut().insert(
        "email".to_string(),
        Parameter::init(Similarity::StartsWith, vec!["value3".to_string()]),
    );

    let filtered = params.remove(vec!["name".to_string(), "email".to_string()]);

    assert_eq!(filtered.inner().len(), 1);
    assert!(!filtered.inner().contains_key("name"));
    assert!(filtered.inner().contains_key("surname"));
    assert!(!filtered.inner().contains_key("email"));
}

#[test]
fn test_parameters_remove_nonexistent_keys() {
    let mut params = Parameters::new();
    params.inner_mut().insert(
        "name".to_string(),
        Parameter::init(Similarity::Contains, vec!["value1".to_string()]),
    );

    let filtered = params.remove(vec!["nonexistent".to_string()]);
    assert_eq!(filtered.inner().len(), 1);
    assert!(filtered.inner().contains_key("name"));
}

#[test]
fn test_parameters_remove_empty_keys() {
    let mut params = Parameters::new();
    params.inner_mut().insert(
        "name".to_string(),
        Parameter::init(Similarity::Contains, vec!["value1".to_string()]),
    );

    let filtered = params.remove(vec![]);
    assert_eq!(filtered.inner().len(), 1);
    assert!(filtered.inner().contains_key("name"));
}


#[test]
fn test_parameters_keep_with_numeric_comparisons() {
    let mut params = Parameters::new();

    params.inner_mut().insert(
        "age".to_string(),
        Parameter::init(
            Similarity::Between,
            vec!["20".to_string(), "30".to_string()],
        ),
    );
    params.inner_mut().insert(
        "price".to_string(),
        Parameter::init(Similarity::Greater, vec!["100".to_string()]),
    );
    params.inner_mut().insert(
        "name".to_string(),
        Parameter::init(Similarity::Contains, vec!["damian".to_string()]),
    );

    let filtered = params.keep(vec!["age".to_string(), "name".to_string()]);

    assert_eq!(filtered.inner().len(), 2);
    assert!(filtered.inner().contains_key("age"));
    assert!(!filtered.inner().contains_key("price"));
    assert!(filtered.inner().contains_key("name"));

    let param = &filtered.inner()["age"];
    assert_eq!(*param.similarity(), Similarity::Between);
    assert_eq!(*param.values(), vec!["20", "30"]);
}

#[test]
fn test_parameters_remove_with_numeric_comparisons() {
    let mut params = Parameters::new();

    params.inner_mut().insert(
        "age".to_string(),
        Parameter::init(
            Similarity::Between,
            vec!["20".to_string(), "30".to_string()],
        ),
    );
    params.inner_mut().insert(
        "price".to_string(),
        Parameter::init(Similarity::Greater, vec!["100".to_string()]),
    );
    params.inner_mut().insert(
        "name".to_string(),
        Parameter::init(Similarity::Contains, vec!["damian".to_string()]),
    );

    let filtered = params.remove(vec!["age".to_string(), "name".to_string()]);

    assert_eq!(filtered.inner().len(), 1);
    assert!(!filtered.inner().contains_key("age"));
    assert!(filtered.inner().contains_key("price"));
    assert!(!filtered.inner().contains_key("name"));

    let param = &filtered.inner()["price"];
    assert_eq!(*param.similarity(), Similarity::Greater);
    assert_eq!(*param.values(), vec!["100"]);
}

// ============================================================================
// ERROR HANDLING TESTS
// ============================================================================

#[test]
fn test_error_invalid_sort_order() {
    let error = SortDirection::from_str("invalid").unwrap_err();
    match error {
        Error::InvalidSortDirection(msg) => assert_eq!(msg, "invalid"),
        _ => panic!("Expected InvalidSortDirection error"),
    }
}

#[test]
fn test_error_invalid_order_field() {
    let error = Order::from_str("invalid").unwrap_err();
    match error {
        Error::InvalidOrderField(msg) => assert_eq!(msg, "invalid"),
        _ => panic!("Expected InvalidOrderField error"),
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
fn test_complex_order_parsing() {
    let sort_str = "date_created:desc,name:asc,surname:asc,email:desc";
    let fields = Order::from_str(sort_str).unwrap();

    assert_eq!(fields.inner().len(), 4);
    assert_eq!(
        fields.inner().get("date_created"),
        Some(&SortDirection::Descending)
    );
    assert_eq!(fields.inner().get("name"), Some(&SortDirection::Ascending));
    assert_eq!(
        fields.inner().get("surname"),
        Some(&SortDirection::Ascending)
    );
    assert_eq!(
        fields.inner().get("email"),
        Some(&SortDirection::Descending)
    );
}

#[test]
fn test_complex_parameters_parsing() {
    let param_str = "name=contains:damian&surname=equals:black,steel,wood&email=starts-with:test&age=ends-with:25";
    let params = Parameters::from_str(param_str).unwrap();

    assert_eq!(params.inner().len(), 4);

    let param = &params.inner()["name"];
    assert_eq!(*param.similarity(), Similarity::Contains);
    assert_eq!(*param.values(), vec!["damian"]);

    let param = &params.inner()["surname"];
    assert_eq!(*param.similarity(), Similarity::Equals);
    assert_eq!(*param.values(), vec!["black", "steel", "wood"]);

    let param = &params.inner()["email"];
    assert_eq!(*param.similarity(), Similarity::StartsWith);
    assert_eq!(*param.values(), vec!["test"]);

    let param = &params.inner()["age"];
    assert_eq!(*param.similarity(), Similarity::EndsWith);
    assert_eq!(*param.values(), vec!["25"]);
}

#[test]
fn test_edge_case_case_sensitivity() {
    // Test that similarity and sort order are case sensitive
    assert!(Similarity::from_str("EQUALS").is_err());
    assert!(Similarity::from_str("Contains").is_err());

    assert!(SortDirection::from_str("ASC").is_err());
    assert!(SortDirection::from_str("DESC").is_err());
}

#[test]
fn test_complex_numeric_parameters_parsing() {
    let param_str = "age=between:20,30&price=greater:100&score=lesser-or-equal:85&rating=greater-or-equal:4.5&discount=lesser:10";
    let params = Parameters::from_str(param_str).unwrap();

    assert_eq!(params.inner().len(), 5);

    let param = &params.inner()["age"];
    assert_eq!(*param.similarity(), Similarity::Between);
    assert_eq!(*param.values(), vec!["20", "30"]);

    let param = &params.inner()["price"];
    assert_eq!(*param.similarity(), Similarity::Greater);
    assert_eq!(*param.values(), vec!["100"]);

    let param = &params.inner()["score"];
    assert_eq!(*param.similarity(), Similarity::LesserOrEqual);
    assert_eq!(*param.values(), vec!["85"]);

    let param = &params.inner()["rating"];
    assert_eq!(*param.similarity(), Similarity::GreaterOrEqual);
    assert_eq!(*param.values(), vec!["4.5"]);

    let param = &params.inner()["discount"];
    assert_eq!(*param.similarity(), Similarity::Lesser);
    assert_eq!(*param.values(), vec!["10"]);
}
