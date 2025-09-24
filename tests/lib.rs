use std::str::FromStr;
use xquery::error::Error;
use xquery::*;

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
}

#[test]
fn test_similarity_constants() {
    assert_eq!(Similarity::EQUALS, "equals");
    assert_eq!(Similarity::CONTAINS, "contains");
    assert_eq!(Similarity::STARTS_WITH, "starts-with");
    assert_eq!(Similarity::ENDS_WITH, "ends-with");
}

#[test]
fn test_similarity_default() {
    assert_eq!(Similarity::default(), Similarity::Contains);
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
}

#[test]
fn test_similarity_from_str_invalid() {
    assert!(Similarity::from_str("invalid").is_err());
    assert!(Similarity::from_str("").is_err());
    assert!(Similarity::from_str("EQUALS").is_err());
    assert!(Similarity::from_str("starts_with").is_err());
    assert!(Similarity::from_str("ends_with").is_err());
}

#[test]
fn test_similarity_to_string() {
    assert_eq!(Similarity::Equals.to_string(), "equals");
    assert_eq!(Similarity::Contains.to_string(), "contains");
    assert_eq!(Similarity::StartsWith.to_string(), "starts-with");
    assert_eq!(Similarity::EndsWith.to_string(), "ends-with");
}

// ============================================================================
// PARAMETER TESTS
// ============================================================================

#[test]
fn test_parameter_new() {
    let param = Parameter::new();
    assert_eq!(param.similarity, Similarity::Contains);
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
    assert_eq!(Parameters::MAX_LIMIT, 100);

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
fn test_query_from_http_with_max_limit() {
    let query = Query::from_http("limit=200".to_string()).unwrap();
    assert_eq!(query.limit, Parameters::MAX_LIMIT);
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
