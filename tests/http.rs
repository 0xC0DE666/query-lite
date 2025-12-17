use query_lite::error::Error;
use query_lite::*;

// ============================================================================
// HTTP FEATURE TESTS
// ============================================================================

#[cfg(feature = "http")]
#[test]
fn test_query_to_http_empty() {
    let query = Query::new();
    let http = query.to_http();
    assert_eq!(http, "limit=50&offset=0");
}

#[cfg(feature = "http")]
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

#[cfg(feature = "http")]
#[test]
fn test_query_to_http_with_sort() {
    let mut query = Query::new();
    query.order.descending("date_created".to_string());

    let http = query.to_http();
    assert!(http.contains("date_created:desc"));
    assert!(http.contains("limit=50"));
    assert!(http.contains("offset=0"));
}

#[cfg(feature = "http")]
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

#[cfg(feature = "http")]
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

#[cfg(feature = "http")]
#[test]
fn test_query_to_http_empty_order() {
    let mut query = Query::new();
    query.order.ascending("".to_string());

    let http = query.to_http();
    assert!(!http.contains(":asc"));
    assert_eq!(http, "limit=50&offset=0");
}

#[cfg(feature = "http")]
#[test]
fn test_query_from_http_empty() {
    let query = Query::from_http("".to_string()).unwrap();
    assert_eq!(query.parameters.inner().len(), 0);
    assert_eq!(query.order.inner().len(), 0);
    assert_eq!(query.limit, Parameters::DEFAULT_LIMIT);
    assert_eq!(query.offset, Parameters::DEFAULT_OFFSET);
}

#[cfg(feature = "http")]
#[test]
fn test_query_from_http_with_question_mark() {
    let query = Query::from_http("?name=contains:damian".to_string()).unwrap();
    assert_eq!(query.parameters.inner().len(), 1);
    assert!(query.parameters.inner().contains_key("name"));
}

#[cfg(feature = "http")]
#[test]
fn test_query_from_http_with_params() {
    let query = Query::from_http("name=contains:damian&surname=equals:black".to_string()).unwrap();
    assert_eq!(query.parameters.inner().len(), 2);
    assert!(query.parameters.inner().contains_key("name"));
    assert!(query.parameters.inner().contains_key("surname"));
}

#[cfg(feature = "http")]
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

#[cfg(feature = "http")]
#[test]
fn test_query_from_http_with_limit() {
    let query = Query::from_http("limit=100".to_string()).unwrap();
    assert_eq!(query.limit, 100);
}

#[cfg(feature = "http")]
#[test]
fn test_query_from_http_with_offset() {
    let query = Query::from_http("offset=20".to_string()).unwrap();
    assert_eq!(query.offset, 20);
}

#[cfg(feature = "http")]
#[test]
fn test_query_from_http_with_invalid_limit() {
    let query = Query::from_http("limit=invalid".to_string()).unwrap();
    assert_eq!(query.limit, Parameters::DEFAULT_LIMIT);
}

#[cfg(feature = "http")]
#[test]
fn test_query_from_http_with_invalid_offset() {
    let query = Query::from_http("offset=invalid".to_string()).unwrap();
    assert_eq!(query.offset, Parameters::DEFAULT_OFFSET);
}

#[cfg(feature = "http")]
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

#[cfg(feature = "http")]
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

#[cfg(feature = "http")]
#[test]
fn test_query_from_http_empty_values() {
    let query = Query::from_http("name=&order=&limit=&offset=".to_string()).unwrap();
    assert_eq!(query.parameters.inner().len(), 0);
    assert_eq!(query.order.inner().len(), 0);
    assert_eq!(query.limit, Parameters::DEFAULT_LIMIT);
    assert_eq!(query.offset, Parameters::DEFAULT_OFFSET);
}

#[cfg(feature = "http")]
#[test]
fn test_query_from_http_invalid() {
    // Missing value
    assert!(Query::from_http("name".to_string()).is_err());

    // Invalid parameter format
    assert!(Query::from_http("name=invalid:damian".to_string()).is_err());

    // Invalid order format
    assert!(Query::from_http("order=invalid".to_string()).is_err());
}

#[cfg(feature = "http")]
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

#[cfg(feature = "http")]
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

#[cfg(feature = "http")]
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

#[cfg(feature = "http")]
#[test]
fn test_error_invalid_search_parameters() {
    let error = Query::from_http("name".to_string()).unwrap_err();
    match error {
        Error::InvalidSearchParameters(msg) => assert_eq!(msg, "name"),
        _ => panic!("Expected InvalidSearchParameters error"),
    }
}

#[cfg(feature = "http")]
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

#[cfg(feature = "http")]
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

#[cfg(feature = "http")]
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

#[cfg(feature = "http")]
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

#[cfg(feature = "http")]
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

#[cfg(feature = "http")]
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

#[cfg(feature = "http")]
#[test]
fn test_edge_case_unicode_characters() {
    let query_str = "name=contains:damian_测试&surname=equals:black,steel,wood";
    let query = Query::from_http(query_str.to_string()).unwrap();

    assert_eq!(query.parameters.inner().len(), 2);

    let param = &query.parameters.inner()["name"];
    assert_eq!(*param.values(), vec!["damian_测试"]);
}

#[cfg(feature = "http")]
#[test]
fn test_edge_case_case_sensitivity() {
    // But parameter keys should be preserved as-is
    let query_str = "Name=contains:damian&NAME=equals:test";
    let query = Query::from_http(query_str.to_string()).unwrap();
    assert_eq!(query.parameters.inner().len(), 2);
    assert!(query.parameters.inner().contains_key("Name"));
    assert!(query.parameters.inner().contains_key("NAME"));
}

#[cfg(feature = "http")]
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

#[cfg(feature = "http")]
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

#[cfg(feature = "http")]
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

#[cfg(feature = "http")]
#[test]
fn test_edge_case_numeric_comparisons_empty_values() {
    let query_str = "age=between:&price=greater:&score=lesser-or-equal:";
    let query = Query::from_http(query_str.to_string()).unwrap();

    // Empty values should be filtered out
    assert_eq!(query.parameters.inner().len(), 0);
}

#[cfg(feature = "http")]
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

#[cfg(feature = "http")]
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

#[cfg(feature = "http")]
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

#[cfg(feature = "http")]
#[test]
fn test_all_new_features_roundtrip() {
    // Build query using all new features
    let mut params = Parameters::new();
    let mut order = Order::new();

    params
        .equals(
            "name".to_string(),
            vec!["john".to_string(), "jane".to_string()],
        )
        .contains("description".to_string(), vec!["rust".to_string()])
        .between("age".to_string(), vec!["20".to_string(), "30".to_string()]);

    order
        .ascending("name".to_string())
        .descending("date_created".to_string());

    let query = Query::init(params, order, 25, 10);

    // Convert to HTTP and back
    let http_string = query.to_http();
    let reconstructed = Query::from_http(http_string).unwrap();

    // Verify all features are preserved
    assert_eq!(reconstructed.parameters.inner().len(), 3);
    assert_eq!(reconstructed.order.inner().len(), 2);
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
        reconstructed.order.inner().get("name"),
        Some(&SortDirection::Ascending)
    );
    assert_eq!(
        reconstructed.order.inner().get("date_created"),
        Some(&SortDirection::Descending)
    );
}

#[cfg(feature = "http")]
#[test]
fn test_query_from_http_normal_parameters() {
    let query = Query::from_http("name=ben&age=20".to_string()).unwrap();

    assert_eq!(query.parameters.inner().len(), 2);

    assert!(query.parameters.inner().contains_key("name"));
    let param = &query.parameters.inner()["name"];
    assert_eq!(*param.similarity(), Similarity::Equals);
    assert_eq!(*param.values(), vec!["ben"]);

    assert!(query.parameters.inner().contains_key("age"));
    let param = &query.parameters.inner()["age"];
    assert_eq!(*param.similarity(), Similarity::Equals);
    assert_eq!(*param.values(), vec!["20"]);
}

#[cfg(feature = "http")]
#[test]
fn test_query_from_http_repeated_parameters() {
    let query = Query::from_http("name=ben&name=john&name=alice".to_string()).unwrap();

    assert_eq!(query.parameters.inner().len(), 1);

    assert!(query.parameters.inner().contains_key("name"));
    let param = &query.parameters.inner()["name"];
    assert_eq!(*param.similarity(), Similarity::Equals);
    assert_eq!(*param.values(), vec!["ben", "john", "alice"]);
}

#[cfg(feature = "http")]
#[test]
fn test_query_from_http_mixed_normal_and_similarity() {
    let query =
        Query::from_http("name=ben&name=john&age=contains:20&status=active".to_string()).unwrap();

    assert_eq!(query.parameters.inner().len(), 3);

    // Normal parameters (repeated)
    assert!(query.parameters.inner().contains_key("name"));
    let param = &query.parameters.inner()["name"];
    assert_eq!(*param.similarity(), Similarity::Equals);
    assert_eq!(*param.values(), vec!["ben", "john"]);

    // Similarity-based parameter
    assert!(query.parameters.inner().contains_key("age"));
    let param = &query.parameters.inner()["age"];
    assert_eq!(*param.similarity(), Similarity::Contains);
    assert_eq!(*param.values(), vec!["20"]);

    // Normal parameter (single)
    assert!(query.parameters.inner().contains_key("status"));
    let param = &query.parameters.inner()["status"];
    assert_eq!(*param.similarity(), Similarity::Equals);
    assert_eq!(*param.values(), vec!["active"]);
}

#[cfg(feature = "http")]
#[test]
fn test_query_from_http_normal_with_special_params() {
    let query =
        Query::from_http("name=ben&age=20&order=date_created:desc&limit=25&offset=10".to_string())
            .unwrap();

    assert_eq!(query.parameters.inner().len(), 2);

    assert!(query.parameters.inner().contains_key("name"));
    let param = &query.parameters.inner()["name"];
    assert_eq!(*param.similarity(), Similarity::Equals);
    assert_eq!(*param.values(), vec!["ben"]);

    assert!(query.parameters.inner().contains_key("age"));
    let param = &query.parameters.inner()["age"];
    assert_eq!(*param.similarity(), Similarity::Equals);
    assert_eq!(*param.values(), vec!["20"]);

    // Check special parameters are handled correctly
    assert_eq!(query.order.inner().len(), 1);
    assert_eq!(
        query.order.inner().get("date_created"),
        Some(&SortDirection::Descending)
    );
    assert_eq!(query.limit, 25);
    assert_eq!(query.offset, 10);
}

#[cfg(feature = "http")]
#[test]
fn test_query_from_http_url_encoded_normal_params() {
    let query = Query::from_http("name=john%20doe&email=test%40example.com".to_string()).unwrap();

    assert_eq!(query.parameters.inner().len(), 2);

    assert!(query.parameters.inner().contains_key("name"));
    let param = &query.parameters.inner()["name"];
    assert_eq!(*param.similarity(), Similarity::Equals);
    assert_eq!(*param.values(), vec!["john doe"]);

    assert!(query.parameters.inner().contains_key("email"));
    let param = &query.parameters.inner()["email"];
    assert_eq!(*param.similarity(), Similarity::Equals);
    assert_eq!(*param.values(), vec!["test@example.com"]);
}

#[cfg(feature = "http")]
#[test]
fn test_query_from_http_repeated_mixed_similarity() {
    let query = Query::from_http("name=ben&name=contains:john&name=alice".to_string()).unwrap();

    assert_eq!(query.parameters.inner().len(), 1);

    assert!(query.parameters.inner().contains_key("name"));
    let param = &query.parameters.inner()["name"];
    // The similarity-based parameter takes precedence
    assert_eq!(*param.similarity(), Similarity::Contains);
    assert_eq!(*param.values(), vec!["john"]);
}

#[cfg(feature = "http")]
#[test]
fn test_query_from_http_empty_normal_values() {
    let query = Query::from_http("name=&age=20&status=".to_string()).unwrap();

    assert_eq!(query.parameters.inner().len(), 1);

    assert!(query.parameters.inner().contains_key("age"));
    let param = &query.parameters.inner()["age"];
    assert_eq!(*param.similarity(), Similarity::Equals);
    assert_eq!(*param.values(), vec!["20"]);
}

#[cfg(feature = "http")]
#[test]
fn test_query_to_http_normal_parameters() {
    let mut query = Query::new();

    query.parameters.inner_mut().insert(
        "name".to_string(),
        Parameter::init(
            Similarity::Equals,
            vec!["ben".to_string(), "john".to_string()],
        ),
    );
    query.parameters.inner_mut().insert(
        "age".to_string(),
        Parameter::init(Similarity::Equals, vec!["20".to_string()]),
    );

    let http = query.to_http();

    assert!(http.contains("name=equals:ben,john"));
    assert!(http.contains("age=equals:20"));
    assert!(http.contains("limit=50"));
    assert!(http.contains("offset=0"));
}

#[cfg(feature = "http")]
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

#[cfg(feature = "http")]
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

