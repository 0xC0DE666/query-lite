use std::str::FromStr;
use query_x::error::Error;
use query_x::*;

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
// SORT FIELD TESTS
// ============================================================================

#[test]
fn test_sort_field_init() {
    let field = SortField::init("name".to_string(), SortOrder::Ascending);
    assert_eq!(field.name, "name");
    assert_eq!(field.order, SortOrder::Ascending);
}

#[test]
fn test_sort_field_from_str_valid() {
    let field = SortField::from_str("name:asc").unwrap();
    assert_eq!(field.name, "name");
    assert_eq!(field.order, SortOrder::Ascending);

    let field = SortField::from_str("date_created:desc").unwrap();
    assert_eq!(field.name, "date_created");
    assert_eq!(field.order, SortOrder::Descending);
}

#[test]
fn test_sort_field_from_str_with_whitespace() {
    let field = SortField::from_str("  name  :  asc  ").unwrap();
    assert_eq!(field.name, "name");
    assert_eq!(field.order, SortOrder::Ascending);
}

#[test]
fn test_sort_field_from_str_invalid() {
    // Empty string
    assert!(SortField::from_str("").is_err());

    // Missing colon
    assert!(SortField::from_str("name").is_err());
    assert!(SortField::from_str("nameasc").is_err());

    // Multiple colons
    assert!(SortField::from_str("name:asc:extra").is_err());

    // Empty name
    assert!(SortField::from_str(":asc").is_err());

    // Empty order
    assert!(SortField::from_str("name:").is_err());

    // Both empty
    assert!(SortField::from_str(":").is_err());

    // Invalid order
    assert!(SortField::from_str("name:invalid").is_err());

    // Only whitespace
    assert!(SortField::from_str("   ").is_err());
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
    assert_eq!(fields.0[0].name, "name");
    assert_eq!(fields.0[0].order, SortOrder::Ascending);
}

#[test]
fn test_sort_fields_from_str_multiple() {
    let fields = SortFields::from_str("date_created:desc,name:asc,surname:asc").unwrap();
    assert_eq!(fields.0.len(), 3);

    assert_eq!(fields.0[0].name, "date_created");
    assert_eq!(fields.0[0].order, SortOrder::Descending);

    assert_eq!(fields.0[1].name, "name");
    assert_eq!(fields.0[1].order, SortOrder::Ascending);

    assert_eq!(fields.0[2].name, "surname");
    assert_eq!(fields.0[2].order, SortOrder::Ascending);
}

#[test]
fn test_sort_fields_from_str_with_whitespace() {
    let fields =
        SortFields::from_str("  date_created:desc  ,  name:asc  ,  surname:asc  ").unwrap();
    assert_eq!(fields.0.len(), 3);
    assert_eq!(fields.0[0].name, "date_created");
    assert_eq!(fields.0[1].name, "name");
    assert_eq!(fields.0[2].name, "surname");
}

#[test]
fn test_sort_fields_from_str_with_empty_fields() {
    let fields = SortFields::from_str("name:asc,,surname:asc").unwrap();
    assert_eq!(fields.0.len(), 2);
    assert_eq!(fields.0[0].name, "name");
    assert_eq!(fields.0[1].name, "surname");
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
// PARAMETER TESTS
// ============================================================================

#[test]
fn test_parameter_new() {
    let param = Parameter::new();
    assert_eq!(param.similarity, Similarity::Equals);
    assert_eq!(param.values.len(), 0);
}

#[test]
fn test_parameter_init() {
    let values = vec!["value1".to_string(), "value2".to_string()];
    let param = Parameter::init(Similarity::Equals, values.clone());
    assert_eq!(param.similarity, Similarity::Equals);
    assert_eq!(param.values, values);
}

#[test]
fn test_parameter_from_str_single_value() {
    let param = Parameter::from_str("contains:damian").unwrap();
    assert_eq!(param.similarity, Similarity::Contains);
    assert_eq!(param.values, vec!["damian"]);
}

#[test]
fn test_parameter_from_str_multiple_values() {
    let param = Parameter::from_str("equals:black,steel,wood").unwrap();
    assert_eq!(param.similarity, Similarity::Equals);
    assert_eq!(param.values, vec!["black", "steel", "wood"]);
}

#[test]
fn test_parameter_from_str_with_whitespace() {
    let param = Parameter::from_str("  contains  :  damian  ").unwrap();
    assert_eq!(param.similarity, Similarity::Contains);
    assert_eq!(param.values, vec!["damian"]);

    let param = Parameter::from_str("equals: black , steel , wood ").unwrap();
    assert_eq!(param.similarity, Similarity::Equals);
    assert_eq!(param.values, vec!["black", "steel", "wood"]);
}

#[test]
fn test_parameter_from_str_empty_values() {
    let param = Parameter::from_str("contains:").unwrap();
    assert_eq!(param.similarity, Similarity::Contains);
    assert_eq!(param.values.len(), 0);
}

#[test]
fn test_parameter_from_str_empty_values_with_commas() {
    let param = Parameter::from_str("contains:,,,").unwrap();
    assert_eq!(param.similarity, Similarity::Contains);
    assert_eq!(param.values.len(), 0);
}

#[test]
fn test_parameter_from_str_mixed_empty_values() {
    let param = Parameter::from_str("contains:value1,,value2,").unwrap();
    assert_eq!(param.similarity, Similarity::Contains);
    assert_eq!(param.values, vec!["value1", "value2"]);
}

#[test]
fn test_parameter_from_str_invalid() {
    // Empty string
    assert!(Parameter::from_str("").is_err());

    // Missing colon
    assert!(Parameter::from_str("contains").is_err());
    assert!(Parameter::from_str("containsdamian").is_err());

    // Multiple colons
    assert!(Parameter::from_str("contains:damian:extra").is_err());

    // Empty similarity
    assert!(Parameter::from_str(":damian").is_err());

    // Invalid similarity
    assert!(Parameter::from_str("invalid:damian").is_err());

    // Only whitespace
    assert!(Parameter::from_str("   ").is_err());
}

#[test]
fn test_parameter_from_str_between_single_range() {
    let param = Parameter::from_str("between:20,30").unwrap();
    assert_eq!(param.similarity, Similarity::Between);
    assert_eq!(param.values, vec!["20", "30"]);
}

#[test]
fn test_parameter_from_str_between_with_whitespace() {
    let param = Parameter::from_str("  between  :  20  ,  30  ").unwrap();
    assert_eq!(param.similarity, Similarity::Between);
    assert_eq!(param.values, vec!["20", "30"]);
}

#[test]
fn test_parameter_from_str_lesser() {
    let param = Parameter::from_str("lesser:100").unwrap();
    assert_eq!(param.similarity, Similarity::Lesser);
    assert_eq!(param.values, vec!["100"]);
}

#[test]
fn test_parameter_from_str_lesser_or_equal() {
    let param = Parameter::from_str("lesser-or-equal:100").unwrap();
    assert_eq!(param.similarity, Similarity::LesserOrEqual);
    assert_eq!(param.values, vec!["100"]);
}

#[test]
fn test_parameter_from_str_greater() {
    let param = Parameter::from_str("greater:50").unwrap();
    assert_eq!(param.similarity, Similarity::Greater);
    assert_eq!(param.values, vec!["50"]);
}

#[test]
fn test_parameter_from_str_greater_or_equal() {
    let param = Parameter::from_str("greater-or-equal:50").unwrap();
    assert_eq!(param.similarity, Similarity::GreaterOrEqual);
    assert_eq!(param.values, vec!["50"]);
}

#[test]
fn test_parameter_from_str_numeric_comparisons_with_whitespace() {
    let param1 = Parameter::from_str("  lesser  :  100  ").unwrap();
    assert_eq!(param1.similarity, Similarity::Lesser);
    assert_eq!(param1.values, vec!["100"]);

    let param2 = Parameter::from_str("  greater-or-equal  :  25  ").unwrap();
    assert_eq!(param2.similarity, Similarity::GreaterOrEqual);
    assert_eq!(param2.values, vec!["25"]);
}

#[test]
fn test_parameter_from_str_between_multiple_values() {
    // Test that between can handle multiple values (though typically it should be exactly 2)
    let param = Parameter::from_str("between:10,20,30").unwrap();
    assert_eq!(param.similarity, Similarity::Between);
    assert_eq!(param.values, vec!["10", "20", "30"]);
}

#[test]
fn test_parameter_from_str_numeric_comparisons_empty_values() {
    let param = Parameter::from_str("lesser:").unwrap();
    assert_eq!(param.similarity, Similarity::Lesser);
    assert_eq!(param.values.len(), 0);

    let param2 = Parameter::from_str("greater:").unwrap();
    assert_eq!(param2.similarity, Similarity::Greater);
    assert_eq!(param2.values.len(), 0);
}

#[test]
fn test_parameter_from_str_between_empty_values() {
    let param = Parameter::from_str("between:").unwrap();
    assert_eq!(param.similarity, Similarity::Between);
    assert_eq!(param.values.len(), 0);
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
    let param = &params.0["name"];
    assert_eq!(param.similarity, Similarity::Contains);
    assert_eq!(param.values, vec!["damian"]);
}

#[test]
fn test_parameters_from_str_multiple() {
    let params =
        Parameters::from_str("name=contains:damian&surname=equals:black,steel,wood").unwrap();
    assert_eq!(params.0.len(), 2);

    assert!(params.0.contains_key("name"));
    let name_param = &params.0["name"];
    assert_eq!(name_param.similarity, Similarity::Contains);
    assert_eq!(name_param.values, vec!["damian"]);

    assert!(params.0.contains_key("surname"));
    let surname_param = &params.0["surname"];
    assert_eq!(surname_param.similarity, Similarity::Equals);
    assert_eq!(surname_param.values, vec!["black", "steel", "wood"]);
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
    let age_param = &params.0["age"];
    assert_eq!(age_param.similarity, Similarity::Between);
    assert_eq!(age_param.values, vec!["20", "30"]);

    assert!(params.0.contains_key("price"));
    let price_param = &params.0["price"];
    assert_eq!(price_param.similarity, Similarity::Greater);
    assert_eq!(price_param.values, vec!["100"]);

    assert!(params.0.contains_key("score"));
    let score_param = &params.0["score"];
    assert_eq!(score_param.similarity, Similarity::LesserOrEqual);
    assert_eq!(score_param.values, vec!["85"]);
}

#[test]
fn test_parameters_from_str_mixed_similarity_types() {
    let params = Parameters::from_str("name=contains:damian&age=between:25,35&price=greater-or-equal:50&status=equals:active").unwrap();
    assert_eq!(params.0.len(), 4);

    assert!(params.0.contains_key("name"));
    let name_param = &params.0["name"];
    assert_eq!(name_param.similarity, Similarity::Contains);
    assert_eq!(name_param.values, vec!["damian"]);

    assert!(params.0.contains_key("age"));
    let age_param = &params.0["age"];
    assert_eq!(age_param.similarity, Similarity::Between);
    assert_eq!(age_param.values, vec!["25", "35"]);

    assert!(params.0.contains_key("price"));
    let price_param = &params.0["price"];
    assert_eq!(price_param.similarity, Similarity::GreaterOrEqual);
    assert_eq!(price_param.values, vec!["50"]);

    assert!(params.0.contains_key("status"));
    let status_param = &params.0["status"];
    assert_eq!(status_param.similarity, Similarity::Equals);
    assert_eq!(status_param.values, vec!["active"]);
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
    let param = Parameter::init(Similarity::Contains, vec!["damian".to_string()]);
    query.parameters.0.insert("name".to_string(), param);

    let http = query.to_http();
    assert!(http.contains("name=contains:damian"));
    assert!(http.contains("limit=50"));
    assert!(http.contains("offset=0"));
}

#[test]
fn test_query_to_http_with_sort() {
    let mut query = Query::new();
    let sort_field = SortField::init("date_created".to_string(), SortOrder::Descending);
    query.sort_fields.0.push(sort_field);

    let http = query.to_http();
    assert!(http.contains("date_created:desc"));
    assert!(http.contains("limit=50"));
    assert!(http.contains("offset=0"));
}

#[test]
fn test_query_to_http_with_params_and_sort() {
    let mut query = Query::new();
    let param = Parameter::init(Similarity::Contains, vec!["damian".to_string()]);
    query.parameters.0.insert("name".to_string(), param);

    let sort_field = SortField::init("date_created".to_string(), SortOrder::Descending);
    query.sort_fields.0.push(sort_field);

    let http = query.to_http();
    assert!(http.contains("name=contains:damian"));
    assert!(http.contains("date_created:desc"));
    assert!(http.contains("limit=50"));
    assert!(http.contains("offset=0"));
}

#[test]
fn test_query_to_http_sort_fields_empty_values() {
    let mut query = Query::new();
    let param = Parameter::init(Similarity::Contains, vec![]);
    query.parameters.0.insert("name".to_string(), param);

    let http = query.to_http();
    assert!(!http.contains("name="));
    assert_eq!(http, "limit=50&offset=0");
}

#[test]
fn test_query_to_http_empty_sort_fields() {
    let mut query = Query::new();
    let sort_field = SortField::init("".to_string(), SortOrder::Ascending);
    query.sort_fields.0.push(sort_field);

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
    assert_eq!(query.sort_fields.0[0].name, "date_created");
    assert_eq!(query.sort_fields.0[0].order, SortOrder::Descending);
    assert_eq!(query.sort_fields.0[1].name, "name");
    assert_eq!(query.sort_fields.0[1].order, SortOrder::Ascending);
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
    assert_eq!(query.sort_fields.0[0].name, "date_created");
    assert_eq!(query.sort_fields.0[0].order, SortOrder::Descending);

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
    assert_eq!(query.sort_fields.0[0].name, "date_created");
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
fn test_query_keep() {
    let mut query = Query::new();
    let param1 = Parameter::init(Similarity::Contains, vec!["value1".to_string()]);
    let param2 = Parameter::init(Similarity::Equals, vec!["value2".to_string()]);
    let param3 = Parameter::init(Similarity::StartsWith, vec!["value3".to_string()]);

    query.parameters.0.insert("name".to_string(), param1);
    query.parameters.0.insert("surname".to_string(), param2);
    query.parameters.0.insert("email".to_string(), param3);

    let filtered = query.keep(vec!["name".to_string(), "email".to_string()]);

    assert_eq!(filtered.parameters.0.len(), 2);
    assert!(filtered.parameters.0.contains_key("name"));
    assert!(!filtered.parameters.0.contains_key("surname"));
    assert!(filtered.parameters.0.contains_key("email"));
}

#[test]
fn test_query_keep_nonexistent_keys() {
    let mut query = Query::new();
    let param = Parameter::init(Similarity::Contains, vec!["value1".to_string()]);
    query.parameters.0.insert("name".to_string(), param);

    let filtered = query.keep(vec!["nonexistent".to_string()]);
    assert_eq!(filtered.parameters.0.len(), 0);
}

#[test]
fn test_query_keep_empty_keys() {
    let mut query = Query::new();
    let param = Parameter::init(Similarity::Contains, vec!["value1".to_string()]);
    query.parameters.0.insert("name".to_string(), param);

    let filtered = query.keep(vec![]);
    assert_eq!(filtered.parameters.0.len(), 0);
}

#[test]
fn test_query_remove() {
    let mut query = Query::new();
    let param1 = Parameter::init(Similarity::Contains, vec!["value1".to_string()]);
    let param2 = Parameter::init(Similarity::Equals, vec!["value2".to_string()]);
    let param3 = Parameter::init(Similarity::StartsWith, vec!["value3".to_string()]);

    query.parameters.0.insert("name".to_string(), param1);
    query.parameters.0.insert("surname".to_string(), param2);
    query.parameters.0.insert("email".to_string(), param3);

    let filtered = query.remove(vec!["name".to_string(), "email".to_string()]);

    assert_eq!(filtered.parameters.0.len(), 1);
    assert!(!filtered.parameters.0.contains_key("name"));
    assert!(filtered.parameters.0.contains_key("surname"));
    assert!(!filtered.parameters.0.contains_key("email"));
}

#[test]
fn test_query_remove_nonexistent_keys() {
    let mut query = Query::new();
    let param = Parameter::init(Similarity::Contains, vec!["value1".to_string()]);
    query.parameters.0.insert("name".to_string(), param);

    let filtered = query.remove(vec!["nonexistent".to_string()]);
    assert_eq!(filtered.parameters.0.len(), 1);
    assert!(filtered.parameters.0.contains_key("name"));
}

#[test]
fn test_query_remove_empty_keys() {
    let mut query = Query::new();
    let param = Parameter::init(Similarity::Contains, vec!["value1".to_string()]);
    query.parameters.0.insert("name".to_string(), param);

    let filtered = query.remove(vec![]);
    assert_eq!(filtered.parameters.0.len(), 1);
    assert!(filtered.parameters.0.contains_key("name"));
}

#[test]
fn test_query_from_http_with_numeric_comparisons() {
    let query = Query::from_http("age=between:20,30&price=greater:100&score=lesser-or-equal:85&order=date_created:desc&limit=25&offset=10".to_string()).unwrap();
    
    assert_eq!(query.parameters.0.len(), 3);
    
    assert!(query.parameters.0.contains_key("age"));
    let age_param = &query.parameters.0["age"];
    assert_eq!(age_param.similarity, Similarity::Between);
    assert_eq!(age_param.values, vec!["20", "30"]);
    
    assert!(query.parameters.0.contains_key("price"));
    let price_param = &query.parameters.0["price"];
    assert_eq!(price_param.similarity, Similarity::Greater);
    assert_eq!(price_param.values, vec!["100"]);
    
    assert!(query.parameters.0.contains_key("score"));
    let score_param = &query.parameters.0["score"];
    assert_eq!(score_param.similarity, Similarity::LesserOrEqual);
    assert_eq!(score_param.values, vec!["85"]);
    
    assert_eq!(query.sort_fields.0.len(), 1);
    assert_eq!(query.sort_fields.0[0].name, "date_created");
    assert_eq!(query.sort_fields.0[0].order, SortOrder::Descending);
    
    assert_eq!(query.limit, 25);
    assert_eq!(query.offset, 10);
}

#[test]
fn test_query_from_http_mixed_similarity_types() {
    let query = Query::from_http("name=contains:damian&age=between:25,35&price=greater-or-equal:50&status=equals:active&order=name:asc&limit=20".to_string()).unwrap();
    
    assert_eq!(query.parameters.0.len(), 4);
    
    assert!(query.parameters.0.contains_key("name"));
    let name_param = &query.parameters.0["name"];
    assert_eq!(name_param.similarity, Similarity::Contains);
    assert_eq!(name_param.values, vec!["damian"]);
    
    assert!(query.parameters.0.contains_key("age"));
    let age_param = &query.parameters.0["age"];
    assert_eq!(age_param.similarity, Similarity::Between);
    assert_eq!(age_param.values, vec!["25", "35"]);
    
    assert!(query.parameters.0.contains_key("price"));
    let price_param = &query.parameters.0["price"];
    assert_eq!(price_param.similarity, Similarity::GreaterOrEqual);
    assert_eq!(price_param.values, vec!["50"]);
    
    assert!(query.parameters.0.contains_key("status"));
    let status_param = &query.parameters.0["status"];
    assert_eq!(status_param.similarity, Similarity::Equals);
    assert_eq!(status_param.values, vec!["active"]);
    
    assert_eq!(query.sort_fields.0.len(), 1);
    assert_eq!(query.sort_fields.0[0].name, "name");
    assert_eq!(query.sort_fields.0[0].order, SortOrder::Ascending);
    
    assert_eq!(query.limit, 20);
}

#[test]
fn test_query_to_http_with_numeric_comparisons() {
    let mut query = Query::new();
    
    let age_param = Parameter::init(Similarity::Between, vec!["20".to_string(), "30".to_string()]);
    query.parameters.0.insert("age".to_string(), age_param);
    
    let price_param = Parameter::init(Similarity::Greater, vec!["100".to_string()]);
    query.parameters.0.insert("price".to_string(), price_param);
    
    let score_param = Parameter::init(Similarity::LesserOrEqual, vec!["85".to_string()]);
    query.parameters.0.insert("score".to_string(), score_param);
    
    let sort_field = SortField::init("date_created".to_string(), SortOrder::Descending);
    query.sort_fields.0.push(sort_field);
    
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
fn test_query_keep_with_numeric_comparisons() {
    let mut query = Query::new();
    
    let age_param = Parameter::init(Similarity::Between, vec!["20".to_string(), "30".to_string()]);
    let price_param = Parameter::init(Similarity::Greater, vec!["100".to_string()]);
    let name_param = Parameter::init(Similarity::Contains, vec!["damian".to_string()]);
    
    query.parameters.0.insert("age".to_string(), age_param);
    query.parameters.0.insert("price".to_string(), price_param);
    query.parameters.0.insert("name".to_string(), name_param);
    
    let filtered = query.keep(vec!["age".to_string(), "name".to_string()]);
    
    assert_eq!(filtered.parameters.0.len(), 2);
    assert!(filtered.parameters.0.contains_key("age"));
    assert!(!filtered.parameters.0.contains_key("price"));
    assert!(filtered.parameters.0.contains_key("name"));
    
    let age_param = &filtered.parameters.0["age"];
    assert_eq!(age_param.similarity, Similarity::Between);
    assert_eq!(age_param.values, vec!["20", "30"]);
}

#[test]
fn test_query_remove_with_numeric_comparisons() {
    let mut query = Query::new();
    
    let age_param = Parameter::init(Similarity::Between, vec!["20".to_string(), "30".to_string()]);
    let price_param = Parameter::init(Similarity::Greater, vec!["100".to_string()]);
    let name_param = Parameter::init(Similarity::Contains, vec!["damian".to_string()]);
    
    query.parameters.0.insert("age".to_string(), age_param);
    query.parameters.0.insert("price".to_string(), price_param);
    query.parameters.0.insert("name".to_string(), name_param);
    
    let filtered = query.remove(vec!["age".to_string(), "name".to_string()]);
    
    assert_eq!(filtered.parameters.0.len(), 1);
    assert!(!filtered.parameters.0.contains_key("age"));
    assert!(filtered.parameters.0.contains_key("price"));
    assert!(!filtered.parameters.0.contains_key("name"));
    
    let price_param = &filtered.parameters.0["price"];
    assert_eq!(price_param.similarity, Similarity::Greater);
    assert_eq!(price_param.values, vec!["100"]);
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
    let error = SortField::from_str("invalid").unwrap_err();
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
    let error = Parameter::from_str("invalid").unwrap_err();
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
    assert_eq!(fields.0[0].name, "date_created");
    assert_eq!(fields.0[0].order, SortOrder::Descending);
    assert_eq!(fields.0[1].name, "name");
    assert_eq!(fields.0[1].order, SortOrder::Ascending);
    assert_eq!(fields.0[2].name, "surname");
    assert_eq!(fields.0[2].order, SortOrder::Ascending);
    assert_eq!(fields.0[3].name, "email");
    assert_eq!(fields.0[3].order, SortOrder::Descending);
}

#[test]
fn test_complex_parameters_parsing() {
    let param_str = "name=contains:damian&surname=equals:black,steel,wood&email=starts-with:test&age=ends-with:25";
    let params = Parameters::from_str(param_str).unwrap();

    assert_eq!(params.0.len(), 4);

    let name_param = &params.0["name"];
    assert_eq!(name_param.similarity, Similarity::Contains);
    assert_eq!(name_param.values, vec!["damian"]);

    let surname_param = &params.0["surname"];
    assert_eq!(surname_param.similarity, Similarity::Equals);
    assert_eq!(surname_param.values, vec!["black", "steel", "wood"]);

    let email_param = &params.0["email"];
    assert_eq!(email_param.similarity, Similarity::StartsWith);
    assert_eq!(email_param.values, vec!["test"]);

    let age_param = &params.0["age"];
    assert_eq!(age_param.similarity, Similarity::EndsWith);
    assert_eq!(age_param.values, vec!["25"]);
}

#[test]
fn test_edge_case_whitespace_handling() {
    // Test various whitespace scenarios
    let query_str = "  name  =  contains  :  damian  &  order  =  date_created  :  desc  ";
    let query = Query::from_http(query_str.to_string()).unwrap();

    assert_eq!(query.parameters.0.len(), 1);
    assert!(query.parameters.0.contains_key("name"));

    let name_param = &query.parameters.0["name"];
    assert_eq!(name_param.similarity, Similarity::Contains);
    assert_eq!(name_param.values, vec!["damian"]);

    assert_eq!(query.sort_fields.0.len(), 1);
    assert_eq!(query.sort_fields.0[0].name, "date_created");
    assert_eq!(query.sort_fields.0[0].order, SortOrder::Descending);
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

    let name_param = &query.parameters.0["name"];
    assert_eq!(name_param.values, vec!["damian test"]);

    let surname_param = &query.parameters.0["surname"];
    assert_eq!(surname_param.values, vec!["black", "steel", "wood"]);
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

    let name_param = &query.parameters.0["name"];
    assert_eq!(name_param.values, vec!["damian_测试"]);
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

    let age_param = &params.0["age"];
    assert_eq!(age_param.similarity, Similarity::Between);
    assert_eq!(age_param.values, vec!["20", "30"]);

    let price_param = &params.0["price"];
    assert_eq!(price_param.similarity, Similarity::Greater);
    assert_eq!(price_param.values, vec!["100"]);

    let score_param = &params.0["score"];
    assert_eq!(score_param.similarity, Similarity::LesserOrEqual);
    assert_eq!(score_param.values, vec!["85"]);

    let rating_param = &params.0["rating"];
    assert_eq!(rating_param.similarity, Similarity::GreaterOrEqual);
    assert_eq!(rating_param.values, vec!["4.5"]);

    let discount_param = &params.0["discount"];
    assert_eq!(discount_param.similarity, Similarity::Lesser);
    assert_eq!(discount_param.values, vec!["10"]);
}

#[test]
fn test_edge_case_numeric_comparisons_with_whitespace() {
    let query_str = "  age  =  between:20,30  &  price  =  greater:100  &  score  =  lesser-or-equal:85  ";
    let query = Query::from_http(query_str.to_string()).unwrap();

    assert_eq!(query.parameters.0.len(), 3);

    let age_param = &query.parameters.0["age"];
    assert_eq!(age_param.similarity, Similarity::Between);
    assert_eq!(age_param.values, vec!["20", "30"]);

    let price_param = &query.parameters.0["price"];
    assert_eq!(price_param.similarity, Similarity::Greater);
    assert_eq!(price_param.values, vec!["100"]);

    let score_param = &query.parameters.0["score"];
    assert_eq!(score_param.similarity, Similarity::LesserOrEqual);
    assert_eq!(score_param.values, vec!["85"]);
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

    let price_param = &query.parameters.0["price"];
    assert_eq!(price_param.similarity, Similarity::Between);
    assert_eq!(price_param.values, vec!["100.50", "200.75"]);

    let discount_param = &query.parameters.0["discount"];
    assert_eq!(discount_param.similarity, Similarity::Greater);
    assert_eq!(discount_param.values, vec!["10%"]);

    let score_param = &query.parameters.0["score"];
    assert_eq!(score_param.similarity, Similarity::LesserOrEqual);
    assert_eq!(score_param.values, vec!["85.5"]);
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
    let param = Parameter::init(Similarity::Equals, vec!["damian".to_string()]);
    query.parameters.0.insert("name".to_string(), param);
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name = ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_equals_multiple_values() {
    let mut query = Query::new();
    let param = Parameter::init(Similarity::Equals, vec!["damian".to_string(), "john".to_string()]);
    query.parameters.0.insert("name".to_string(), param);
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name IN (?, ?) LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_equals_null() {
    let mut query = Query::new();
    let param = Parameter::init(Similarity::Equals, vec!["null".to_string()]);
    query.parameters.0.insert("name".to_string(), param);
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name IS ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_contains() {
    let mut query = Query::new();
    let param = Parameter::init(Similarity::Contains, vec!["damian".to_string()]);
    query.parameters.0.insert("name".to_string(), param);
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name LIKE ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_contains_multiple_values() {
    let mut query = Query::new();
    let param = Parameter::init(Similarity::Contains, vec!["damian".to_string(), "john".to_string()]);
    query.parameters.0.insert("name".to_string(), param);
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE (name LIKE ? OR name LIKE ?) LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_starts_with() {
    let mut query = Query::new();
    let param = Parameter::init(Similarity::StartsWith, vec!["damian".to_string()]);
    query.parameters.0.insert("name".to_string(), param);
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name LIKE ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_ends_with() {
    let mut query = Query::new();
    let param = Parameter::init(Similarity::EndsWith, vec!["damian".to_string()]);
    query.parameters.0.insert("name".to_string(), param);
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name LIKE ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_between() {
    let mut query = Query::new();
    let param = Parameter::init(Similarity::Between, vec!["20".to_string(), "30".to_string()]);
    query.parameters.0.insert("age".to_string(), param);
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE age BETWEEN ? AND ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_lesser() {
    let mut query = Query::new();
    let param = Parameter::init(Similarity::Lesser, vec!["100".to_string()]);
    query.parameters.0.insert("price".to_string(), param);
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE price < ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_lesser_or_equal() {
    let mut query = Query::new();
    let param = Parameter::init(Similarity::LesserOrEqual, vec!["100".to_string()]);
    query.parameters.0.insert("price".to_string(), param);
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE price <= ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_greater() {
    let mut query = Query::new();
    let param = Parameter::init(Similarity::Greater, vec!["50".to_string()]);
    query.parameters.0.insert("price".to_string(), param);
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE price > ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_greater_or_equal() {
    let mut query = Query::new();
    let param = Parameter::init(Similarity::GreaterOrEqual, vec!["50".to_string()]);
    query.parameters.0.insert("price".to_string(), param);
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE price >= ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_order_by() {
    let mut query = Query::new();
    let sort_field = SortField::init("date_created".to_string(), SortOrder::Descending);
    query.sort_fields.0.push(sort_field);
    
    let sql = query.to_sql();
    assert_eq!(sql, "ORDER BY date_created DESC LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_multiple_order_by() {
    let mut query = Query::new();
    let sort_field1 = SortField::init("date_created".to_string(), SortOrder::Descending);
    let sort_field2 = SortField::init("name".to_string(), SortOrder::Ascending);
    query.sort_fields.0.push(sort_field1);
    query.sort_fields.0.push(sort_field2);
    
    let sql = query.to_sql();
    assert_eq!(sql, "ORDER BY date_created DESC, name ASC LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_complex() {
    let mut query = Query::new();
    
    // Add multiple parameters
    let name_param = Parameter::init(Similarity::Contains, vec!["damian".to_string()]);
    query.parameters.0.insert("name".to_string(), name_param);
    
    let age_param = Parameter::init(Similarity::Between, vec!["20".to_string(), "30".to_string()]);
    query.parameters.0.insert("age".to_string(), age_param);
    
    let price_param = Parameter::init(Similarity::Greater, vec!["100".to_string()]);
    query.parameters.0.insert("price".to_string(), price_param);
    
    // Add sorting
    let sort_field = SortField::init("date_created".to_string(), SortOrder::Descending);
    query.sort_fields.0.push(sort_field);
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name LIKE ? AND age BETWEEN ? AND ? AND price > ? ORDER BY date_created DESC LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_empty_parameters() {
    let mut query = Query::new();
    let param = Parameter::init(Similarity::Contains, vec![]);
    query.parameters.0.insert("name".to_string(), param);
    
    let sql = query.to_sql();
    assert_eq!(sql, "LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_empty_sort_fields() {
    let mut query = Query::new();
    let sort_field = SortField::init("".to_string(), SortOrder::Ascending);
    query.sort_fields.0.push(sort_field);
    
    let sql = query.to_sql();
    assert_eq!(sql, "LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_numeric_comparisons_multiple_values() {
    let mut query = Query::new();
    let param = Parameter::init(Similarity::Greater, vec!["50".to_string(), "100".to_string()]);
    query.parameters.0.insert("price".to_string(), param);
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE (price > ? OR price > ?) LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_between_invalid_values() {
    let mut query = Query::new();
    let param = Parameter::init(Similarity::Between, vec!["20".to_string()]);
    query.parameters.0.insert("age".to_string(), param);
    
    let sql = query.to_sql();
    assert_eq!(sql, "LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_between_multiple_pairs() {
    let mut query = Query::new();
    let param = Parameter::init(Similarity::Between, vec!["10".to_string(), "20".to_string(), "30".to_string(), "40".to_string()]);
    query.parameters.0.insert("age".to_string(), param);
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE (age BETWEEN ? AND ? OR age BETWEEN ? AND ?) LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_between_odd_values_ignored() {
    let mut query = Query::new();
    let param = Parameter::init(Similarity::Between, vec!["10".to_string(), "20".to_string(), "30".to_string(), "40".to_string(), "50".to_string()]);
    query.parameters.0.insert("age".to_string(), param);
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE (age BETWEEN ? AND ? OR age BETWEEN ? AND ?) LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_between_three_pairs() {
    let mut query = Query::new();
    let param = Parameter::init(Similarity::Between, vec!["10".to_string(), "20".to_string(), "30".to_string(), "40".to_string(), "50".to_string(), "60".to_string()]);
    query.parameters.0.insert("age".to_string(), param);
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE (age BETWEEN ? AND ? OR age BETWEEN ? AND ? OR age BETWEEN ? AND ?) LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_between_single_pair() {
    let mut query = Query::new();
    let param = Parameter::init(Similarity::Between, vec!["20".to_string(), "30".to_string()]);
    query.parameters.0.insert("age".to_string(), param);
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE age BETWEEN ? AND ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_between_empty_values() {
    let mut query = Query::new();
    let param = Parameter::init(Similarity::Between, vec![]);
    query.parameters.0.insert("age".to_string(), param);
    
    let sql = query.to_sql();
    assert_eq!(sql, "LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_between_complex_with_other_conditions() {
    let mut query = Query::new();
    
    // Add between with multiple pairs
    let age_param = Parameter::init(Similarity::Between, vec!["10".to_string(), "20".to_string(), "30".to_string(), "40".to_string(), "50".to_string()]);
    query.parameters.0.insert("age".to_string(), age_param);
    
    // Add other condition
    let name_param = Parameter::init(Similarity::Contains, vec!["damian".to_string()]);
    query.parameters.0.insert("name".to_string(), name_param);
    
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
    let name_param = &query.parameters.0["name"];
    assert_eq!(name_param.similarity, Similarity::Equals);
    assert_eq!(name_param.values, vec!["ben"]);
    
    assert!(query.parameters.0.contains_key("age"));
    let age_param = &query.parameters.0["age"];
    assert_eq!(age_param.similarity, Similarity::Equals);
    assert_eq!(age_param.values, vec!["20"]);
}

#[test]
fn test_query_from_http_repeated_parameters() {
    let query = Query::from_http("name=ben&name=john&name=alice".to_string()).unwrap();
    
    assert_eq!(query.parameters.0.len(), 1);
    
    assert!(query.parameters.0.contains_key("name"));
    let name_param = &query.parameters.0["name"];
    assert_eq!(name_param.similarity, Similarity::Equals);
    assert_eq!(name_param.values, vec!["ben", "john", "alice"]);
}

#[test]
fn test_query_from_http_mixed_normal_and_similarity() {
    let query = Query::from_http("name=ben&name=john&age=contains:20&status=active".to_string()).unwrap();
    
    assert_eq!(query.parameters.0.len(), 3);
    
    // Normal parameters (repeated)
    assert!(query.parameters.0.contains_key("name"));
    let name_param = &query.parameters.0["name"];
    assert_eq!(name_param.similarity, Similarity::Equals);
    assert_eq!(name_param.values, vec!["ben", "john"]);
    
    // Similarity-based parameter
    assert!(query.parameters.0.contains_key("age"));
    let age_param = &query.parameters.0["age"];
    assert_eq!(age_param.similarity, Similarity::Contains);
    assert_eq!(age_param.values, vec!["20"]);
    
    // Normal parameter (single)
    assert!(query.parameters.0.contains_key("status"));
    let status_param = &query.parameters.0["status"];
    assert_eq!(status_param.similarity, Similarity::Equals);
    assert_eq!(status_param.values, vec!["active"]);
}

#[test]
fn test_query_from_http_normal_with_special_params() {
    let query = Query::from_http("name=ben&age=20&order=date_created:desc&limit=25&offset=10".to_string()).unwrap();
    
    assert_eq!(query.parameters.0.len(), 2);
    
    assert!(query.parameters.0.contains_key("name"));
    let name_param = &query.parameters.0["name"];
    assert_eq!(name_param.similarity, Similarity::Equals);
    assert_eq!(name_param.values, vec!["ben"]);
    
    assert!(query.parameters.0.contains_key("age"));
    let age_param = &query.parameters.0["age"];
    assert_eq!(age_param.similarity, Similarity::Equals);
    assert_eq!(age_param.values, vec!["20"]);
    
    // Check special parameters are handled correctly
    assert_eq!(query.sort_fields.0.len(), 1);
    assert_eq!(query.sort_fields.0[0].name, "date_created");
    assert_eq!(query.sort_fields.0[0].order, SortOrder::Descending);
    assert_eq!(query.limit, 25);
    assert_eq!(query.offset, 10);
}

#[test]
fn test_query_from_http_url_encoded_normal_params() {
    let query = Query::from_http("name=john%20doe&email=test%40example.com".to_string()).unwrap();
    
    assert_eq!(query.parameters.0.len(), 2);
    
    assert!(query.parameters.0.contains_key("name"));
    let name_param = &query.parameters.0["name"];
    assert_eq!(name_param.similarity, Similarity::Equals);
    assert_eq!(name_param.values, vec!["john doe"]);
    
    assert!(query.parameters.0.contains_key("email"));
    let email_param = &query.parameters.0["email"];
    assert_eq!(email_param.similarity, Similarity::Equals);
    assert_eq!(email_param.values, vec!["test@example.com"]);
}

#[test]
fn test_query_from_http_repeated_mixed_similarity() {
    let query = Query::from_http("name=ben&name=contains:john&name=alice".to_string()).unwrap();
    
    assert_eq!(query.parameters.0.len(), 1);
    
    assert!(query.parameters.0.contains_key("name"));
    let name_param = &query.parameters.0["name"];
    // The similarity-based parameter takes precedence
    assert_eq!(name_param.similarity, Similarity::Contains);
    assert_eq!(name_param.values, vec!["john"]);
}

#[test]
fn test_query_from_http_empty_normal_values() {
    let query = Query::from_http("name=&age=20&status=".to_string()).unwrap();
    
    assert_eq!(query.parameters.0.len(), 1);
    
    assert!(query.parameters.0.contains_key("age"));
    let age_param = &query.parameters.0["age"];
    assert_eq!(age_param.similarity, Similarity::Equals);
    assert_eq!(age_param.values, vec!["20"]);
}

#[test]
fn test_query_to_http_normal_parameters() {
    let mut query = Query::new();
    
    let name_param = Parameter::init(Similarity::Equals, vec!["ben".to_string(), "john".to_string()]);
    query.parameters.0.insert("name".to_string(), name_param);
    
    let age_param = Parameter::init(Similarity::Equals, vec!["20".to_string()]);
    query.parameters.0.insert("age".to_string(), age_param);
    
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
    
    let name_param = Parameter::init(Similarity::Equals, vec!["ben".to_string(), "john".to_string()]);
    query.parameters.0.insert("name".to_string(), name_param);
    
    let age_param = Parameter::init(Similarity::Equals, vec!["20".to_string()]);
    query.parameters.0.insert("age".to_string(), age_param);
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name IN (?, ?) AND age = ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_mixed_normal_and_similarity() {
    let mut query = Query::new();
    
    let name_param = Parameter::init(Similarity::Equals, vec!["ben".to_string(), "john".to_string()]);
    query.parameters.0.insert("name".to_string(), name_param);
    
    let age_param = Parameter::init(Similarity::Contains, vec!["20".to_string()]);
    query.parameters.0.insert("age".to_string(), age_param);
    
    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name IN (?, ?) AND age LIKE ? LIMIT ? OFFSET ?");
}
