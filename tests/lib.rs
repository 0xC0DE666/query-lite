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
fn test_query_to_http_empty() {
    let query = Query::new();
    let http = query.to_http();
    assert_eq!(http, "limit=50&offset=0");
}

#[test]
fn test_query_to_http_with_params() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "name".to_string(),
        Parameter::init(Similarity::Contains, vec!["damian".to_string()]),
    );

    let http = query.to_http();
    assert!(http.contains("name=contains:damian"));
    assert!(http.contains("limit=50"));
    assert!(http.contains("offset=0"));
}

#[test]
fn test_query_to_http_with_sort() {
    let mut query = Query::new();
    query.order.descending("date_created".to_string());

    let http = query.to_http();
    assert!(http.contains("date_created:desc"));
    assert!(http.contains("limit=50"));
    assert!(http.contains("offset=0"));
}

#[test]
fn test_query_to_http_with_params_and_sort() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "name".to_string(),
        Parameter::init(Similarity::Contains, vec!["damian".to_string()]),
    );

    query.order.descending("date_created".to_string());

    let http = query.to_http();
    assert!(http.contains("name=contains:damian"));
    assert!(http.contains("date_created:desc"));
    assert!(http.contains("limit=50"));
    assert!(http.contains("offset=0"));
}

#[test]
fn test_query_to_http_order_empty_values() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "name".to_string(),
        Parameter::init(Similarity::Contains, vec![]),
    );

    let http = query.to_http();
    assert!(!http.contains("name="));
    assert_eq!(http, "limit=50&offset=0");
}

#[test]
fn test_query_to_http_empty_order() {
    let mut query = Query::new();
    query.order.ascending("".to_string());

    let http = query.to_http();
    assert!(!http.contains(":asc"));
    assert_eq!(http, "limit=50&offset=0");
}

#[test]
fn test_query_from_http_empty() {
    let query = Query::from_http("".to_string()).unwrap();
    assert_eq!(query.parameters.inner().len(), 0);
    assert_eq!(query.order.inner().len(), 0);
    assert_eq!(query.limit, Parameters::DEFAULT_LIMIT);
    assert_eq!(query.offset, Parameters::DEFAULT_OFFSET);
}

#[test]
fn test_query_from_http_with_question_mark() {
    let query = Query::from_http("?name=contains:damian".to_string()).unwrap();
    assert_eq!(query.parameters.inner().len(), 1);
    assert!(query.parameters.inner().contains_key("name"));
}

#[test]
fn test_query_from_http_with_params() {
    let query = Query::from_http("name=contains:damian&surname=equals:black".to_string()).unwrap();
    assert_eq!(query.parameters.inner().len(), 2);
    assert!(query.parameters.inner().contains_key("name"));
    assert!(query.parameters.inner().contains_key("surname"));
}

#[test]
fn test_query_from_http_with_order() {
    let query = Query::from_http("order=date_created:desc,name:asc".to_string()).unwrap();
    assert_eq!(query.order.inner().len(), 2);
    assert_eq!(
        query.order.inner().get("date_created"),
        Some(&SortDirection::Descending)
    );
    assert_eq!(
        query.order.inner().get("name"),
        Some(&SortDirection::Ascending)
    );
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

    assert_eq!(query.parameters.inner().len(), 2);
    assert!(query.parameters.inner().contains_key("name"));
    assert!(query.parameters.inner().contains_key("surname"));

    assert_eq!(query.order.inner().len(), 1);
    assert_eq!(
        query.order.inner().get("date_created"),
        Some(&SortDirection::Descending)
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
    assert_eq!(query.parameters.inner().len(), 1);
    assert!(query.parameters.inner().contains_key("name"));
    assert_eq!(query.order.inner().len(), 1);
    assert_eq!(
        query.order.inner().get("date_created"),
        Some(&SortDirection::Descending)
    );
}

#[test]
fn test_query_from_http_empty_values() {
    let query = Query::from_http("name=&order=&limit=&offset=".to_string()).unwrap();
    assert_eq!(query.parameters.inner().len(), 0);
    assert_eq!(query.order.inner().len(), 0);
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
fn test_query_from_http_with_numeric_comparisons() {
    let query = Query::from_http("age=between:20,30&price=greater:100&score=lesser-or-equal:85&order=date_created:desc&limit=25&offset=10".to_string()).unwrap();

    assert_eq!(query.parameters.inner().len(), 3);

    assert!(query.parameters.inner().contains_key("age"));
    let param = &query.parameters.inner()["age"];
    assert_eq!(*param.similarity(), Similarity::Between);
    assert_eq!(*param.values(), vec!["20", "30"]);

    assert!(query.parameters.inner().contains_key("price"));
    let param = &query.parameters.inner()["price"];
    assert_eq!(*param.similarity(), Similarity::Greater);
    assert_eq!(*param.values(), vec!["100"]);

    assert!(query.parameters.inner().contains_key("score"));
    let param = &query.parameters.inner()["score"];
    assert_eq!(*param.similarity(), Similarity::LesserOrEqual);
    assert_eq!(*param.values(), vec!["85"]);

    assert_eq!(query.order.inner().len(), 1);
    assert_eq!(
        query.order.inner().get("date_created"),
        Some(&SortDirection::Descending)
    );

    assert_eq!(query.limit, 25);
    assert_eq!(query.offset, 10);
}

#[test]
fn test_query_from_http_mixed_similarity_types() {
    let query = Query::from_http("name=contains:damian&age=between:25,35&price=greater-or-equal:50&status=equals:active&order=name:asc&limit=20".to_string()).unwrap();

    assert_eq!(query.parameters.inner().len(), 4);

    assert!(query.parameters.inner().contains_key("name"));
    let param = &query.parameters.inner()["name"];
    assert_eq!(*param.similarity(), Similarity::Contains);
    assert_eq!(*param.values(), vec!["damian"]);

    assert!(query.parameters.inner().contains_key("age"));
    let param = &query.parameters.inner()["age"];
    assert_eq!(*param.similarity(), Similarity::Between);
    assert_eq!(*param.values(), vec!["25", "35"]);

    assert!(query.parameters.inner().contains_key("price"));
    let param = &query.parameters.inner()["price"];
    assert_eq!(*param.similarity(), Similarity::GreaterOrEqual);
    assert_eq!(*param.values(), vec!["50"]);

    assert!(query.parameters.inner().contains_key("status"));
    let param = &query.parameters.inner()["status"];
    assert_eq!(*param.similarity(), Similarity::Equals);
    assert_eq!(*param.values(), vec!["active"]);

    assert_eq!(query.order.inner().len(), 1);
    assert_eq!(
        query.order.inner().get("name"),
        Some(&SortDirection::Ascending)
    );

    assert_eq!(query.limit, 20);
}

#[test]
fn test_query_to_http_with_numeric_comparisons() {
    let mut query = Query::new();

    query.parameters.inner_mut().insert(
        "age".to_string(),
        Parameter::init(
            Similarity::Between,
            vec!["20".to_string(), "30".to_string()],
        ),
    );
    query.parameters.inner_mut().insert(
        "price".to_string(),
        Parameter::init(Similarity::Greater, vec!["100".to_string()]),
    );
    query.parameters.inner_mut().insert(
        "score".to_string(),
        Parameter::init(Similarity::LesserOrEqual, vec!["85".to_string()]),
    );

    query.order.descending("date_created".to_string());

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
fn test_edge_case_whitespace_handling() {
    // Test various whitespace scenarios
    let query_str = "  name  =  contains  :  damian  &  order  =  date_created  :  desc  ";
    let query = Query::from_http(query_str.to_string()).unwrap();

    assert_eq!(query.parameters.inner().len(), 1);
    assert!(query.parameters.inner().contains_key("name"));

    let param = &query.parameters.inner()["name"];
    assert_eq!(*param.similarity(), Similarity::Contains);
    assert_eq!(*param.values(), vec!["damian"]);

    assert_eq!(query.order.inner().len(), 1);
    assert_eq!(
        query.order.inner().get("date_created"),
        Some(&SortDirection::Descending)
    );
}

#[test]
fn test_edge_case_empty_and_whitespace_values() {
    let query_str = "name=contains:&surname=equals:,,&order=:desc&limit=&offset=";
    let query = Query::from_http(query_str.to_string()).unwrap();

    // Empty values should be filtered out
    assert_eq!(query.parameters.inner().len(), 0);
    assert_eq!(query.order.inner().len(), 0);
    assert_eq!(query.limit, Parameters::DEFAULT_LIMIT);
    assert_eq!(query.offset, Parameters::DEFAULT_OFFSET);
}

#[test]
fn test_edge_case_special_characters_in_values() {
    let query_str = "name=contains:damian%20test&surname=equals:black,steel,wood";
    let query = Query::from_http(query_str.to_string()).unwrap();

    assert_eq!(query.parameters.inner().len(), 2);

    let param = &query.parameters.inner()["name"];
    assert_eq!(*param.values(), vec!["damian test"]);

    let param = &query.parameters.inner()["surname"];
    assert_eq!(*param.values(), vec!["black", "steel", "wood"]);
}

#[test]
fn test_edge_case_very_long_query() {
    let mut query_str = String::new();
    for i in 0..100 {
        query_str.push_str(&format!("param{}={}:value{}&", i, "contains", i));
    }
    query_str.push_str("order=name:asc&limit=50&offset=0");

    let query = Query::from_http(query_str).unwrap();
    assert_eq!(query.parameters.inner().len(), 100);
    assert_eq!(query.order.inner().len(), 1);
    assert_eq!(query.limit, 50);
    assert_eq!(query.offset, 0);
}

#[test]
fn test_edge_case_unicode_characters() {
    let query_str = "name=contains:damian_测试&surname=equals:black,steel,wood";
    let query = Query::from_http(query_str.to_string()).unwrap();

    assert_eq!(query.parameters.inner().len(), 2);

    let param = &query.parameters.inner()["name"];
    assert_eq!(*param.values(), vec!["damian_测试"]);
}

#[test]
fn test_edge_case_case_sensitivity() {
    // Test that similarity and sort order are case sensitive
    assert!(Similarity::from_str("EQUALS").is_err());
    assert!(Similarity::from_str("Contains").is_err());

    assert!(SortDirection::from_str("ASC").is_err());
    assert!(SortDirection::from_str("DESC").is_err());

    // But parameter keys should be preserved as-is
    let query_str = "Name=contains:damian&NAME=equals:test";
    let query = Query::from_http(query_str.to_string()).unwrap();
    assert_eq!(query.parameters.inner().len(), 2);
    assert!(query.parameters.inner().contains_key("Name"));
    assert!(query.parameters.inner().contains_key("NAME"));
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

#[test]
fn test_edge_case_numeric_comparisons_with_whitespace() {
    let query_str =
        "  age  =  between:20,30  &  price  =  greater:100  &  score  =  lesser-or-equal:85  ";
    let query = Query::from_http(query_str.to_string()).unwrap();

    assert_eq!(query.parameters.inner().len(), 3);

    let param = &query.parameters.inner()["age"];
    assert_eq!(*param.similarity(), Similarity::Between);
    assert_eq!(*param.values(), vec!["20", "30"]);

    let param = &query.parameters.inner()["price"];
    assert_eq!(*param.similarity(), Similarity::Greater);
    assert_eq!(*param.values(), vec!["100"]);

    let param = &query.parameters.inner()["score"];
    assert_eq!(*param.similarity(), Similarity::LesserOrEqual);
    assert_eq!(*param.values(), vec!["85"]);
}

#[test]
fn test_edge_case_numeric_comparisons_empty_values() {
    let query_str = "age=between:&price=greater:&score=lesser-or-equal:";
    let query = Query::from_http(query_str.to_string()).unwrap();

    // Empty values should be filtered out
    assert_eq!(query.parameters.inner().len(), 0);
}

#[test]
fn test_edge_case_numeric_comparisons_mixed_empty_values() {
    let query_str = "age=between:20,30&price=greater:&score=lesser-or-equal:85&name=contains:";
    let query = Query::from_http(query_str.to_string()).unwrap();

    // Only parameters with values should be included
    assert_eq!(query.parameters.inner().len(), 2);
    assert!(query.parameters.inner().contains_key("age"));
    assert!(query.parameters.inner().contains_key("score"));
    assert!(!query.parameters.inner().contains_key("price"));
    assert!(!query.parameters.inner().contains_key("name"));
}

#[test]
fn test_edge_case_numeric_comparisons_special_characters() {
    let query_str = "price=between:100.50,200.75&discount=greater:10%&score=lesser-or-equal:85.5";
    let query = Query::from_http(query_str.to_string()).unwrap();

    assert_eq!(query.parameters.inner().len(), 3);

    let param = &query.parameters.inner()["price"];
    assert_eq!(*param.similarity(), Similarity::Between);
    assert_eq!(*param.values(), vec!["100.50", "200.75"]);

    let param = &query.parameters.inner()["discount"];
    assert_eq!(*param.similarity(), Similarity::Greater);
    assert_eq!(*param.values(), vec!["10%"]);

    let param = &query.parameters.inner()["score"];
    assert_eq!(*param.similarity(), Similarity::LesserOrEqual);
    assert_eq!(*param.values(), vec!["85.5"]);
}
