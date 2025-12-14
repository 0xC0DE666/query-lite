use query_lite::error::Error;
use query_lite::*;
use std::str::FromStr;

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
    query.parameters.inner_mut().insert(
        "name".to_string(),
        Parameter::init(Similarity::Equals, vec!["damian".to_string()]),
    );

    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name = ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_equals_multiple_values() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "name".to_string(),
        Parameter::init(
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
    query.parameters.inner_mut().insert(
        "name".to_string(),
        Parameter::init(Similarity::Equals, vec![sql::NULL.to_string()]),
    );

    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name IS ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_contains() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "name".to_string(),
        Parameter::init(Similarity::Contains, vec!["damian".to_string()]),
    );

    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name LIKE ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_contains_multiple_values() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "name".to_string(),
        Parameter::init(
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
    query.parameters.inner_mut().insert(
        "name".to_string(),
        Parameter::init(Similarity::StartsWith, vec!["damian".to_string()]),
    );

    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name LIKE ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_ends_with() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "name".to_string(),
        Parameter::init(Similarity::EndsWith, vec!["damian".to_string()]),
    );

    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name LIKE ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_between() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "age".to_string(),
        Parameter::init(
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
    query.parameters.inner_mut().insert(
        "price".to_string(),
        Parameter::init(Similarity::Lesser, vec!["100".to_string()]),
    );

    let sql = query.to_sql();
    assert_eq!(sql, "WHERE price < ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_lesser_or_equal() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "price".to_string(),
        Parameter::init(Similarity::LesserOrEqual, vec!["100".to_string()]),
    );

    let sql = query.to_sql();
    assert_eq!(sql, "WHERE price <= ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_greater() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "price".to_string(),
        Parameter::init(Similarity::Greater, vec!["50".to_string()]),
    );

    let sql = query.to_sql();
    assert_eq!(sql, "WHERE price > ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_greater_or_equal() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "price".to_string(),
        Parameter::init(Similarity::GreaterOrEqual, vec!["50".to_string()]),
    );

    let sql = query.to_sql();
    assert_eq!(sql, "WHERE price >= ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_order_by() {
    let mut query = Query::new();
    query.order.descending("date_created".to_string());

    let sql = query.to_sql();
    assert_eq!(sql, "ORDER BY date_created DESC LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_multiple_order_by() {
    let mut query = Query::new();
    query.order.descending("date_created".to_string());
    query.order.ascending("name".to_string());

    let sql = query.to_sql();
    assert_eq!(sql, "ORDER BY date_created DESC, name ASC LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_complex() {
    let mut query = Query::new();

    // Add multiple parameters
    query.parameters.inner_mut().insert(
        "name".to_string(),
        Parameter::init(Similarity::Contains, vec!["damian".to_string()]),
    );
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

    // Add sorting
    query.order.descending("date_created".to_string());

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
    query.parameters.inner_mut().insert(
        "name".to_string(),
        Parameter::init(Similarity::Contains, vec![]),
    );

    let sql = query.to_sql();
    assert_eq!(sql, "LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_with_empty_order() {
    let mut query = Query::new();
    query.order.ascending("".to_string());

    let sql = query.to_sql();
    assert_eq!(sql, "LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_numeric_comparisons_multiple_values() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "price".to_string(),
        Parameter::init(
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
    query.parameters.inner_mut().insert(
        "age".to_string(),
        Parameter::init(Similarity::Between, vec!["20".to_string()]),
    );

    let sql = query.to_sql();
    assert_eq!(sql, "LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_between_multiple_pairs() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "age".to_string(),
        Parameter::init(
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
    query.parameters.inner_mut().insert(
        "age".to_string(),
        Parameter::init(
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
    query.parameters.inner_mut().insert(
        "age".to_string(),
        Parameter::init(
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
    query.parameters.inner_mut().insert(
        "age".to_string(),
        Parameter::init(
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
    query.parameters.inner_mut().insert(
        "age".to_string(),
        Parameter::init(Similarity::Between, vec![]),
    );

    let sql = query.to_sql();
    assert_eq!(sql, "LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_between_complex_with_other_conditions() {
    let mut query = Query::new();

    // Add between with multiple pairs
    query.parameters.inner_mut().insert(
        "age".to_string(),
        Parameter::init(
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
    query.parameters.inner_mut().insert(
        "name".to_string(),
        Parameter::init(Similarity::Contains, vec!["damian".to_string()]),
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
    let param: Parameter = Parameter::init(Similarity::Contains, vec!["test".to_string()]);

    // Test tuple access (backward compatibility)
    assert_eq!(*param.similarity(), Similarity::Contains);
    assert_eq!(*param.values(), vec!["test"]);
}

#[test]
fn test_parameter_get_trait_similarity() {
    let param: Parameter = Parameter::init(
        Similarity::Equals,
        vec!["value1".to_string(), "value2".to_string()],
    );

    // Test trait method access
    assert_eq!(param.similarity(), &Similarity::Equals);

    // Test with different similarity types
    let param2: Parameter = Parameter::init(
        Similarity::Between,
        vec!["10".to_string(), "20".to_string()],
    );
    assert_eq!(param2.similarity(), &Similarity::Between);

    let param3: Parameter = Parameter::init(Similarity::GreaterOrEqual, vec!["100".to_string()]);
    assert_eq!(param3.similarity(), &Similarity::GreaterOrEqual);
}

#[test]
fn test_parameter_get_trait_values() {
    let param: Parameter = Parameter::init(
        Similarity::Contains,
        vec!["value1".to_string(), "value2".to_string()],
    );

    // Test trait method access
    assert_eq!(
        param.values(),
        &vec!["value1".to_string(), "value2".to_string()]
    );

    // Test with empty values
    let param2: Parameter = Parameter::init(Similarity::Equals, vec![]);
    assert_eq!(param2.values(), &vec![] as &Vec<String>);

    // Test with single value
    let param3: Parameter = Parameter::init(Similarity::StartsWith, vec!["prefix".to_string()]);
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
        let param: Parameter = Parameter::init(similarity.clone(), vec!["test".to_string()]);
        assert_eq!(param.similarity(), &similarity);
        assert_eq!(param.values(), &vec!["test".to_string()]);
    }
}

#[test]
fn test_parameter_get_trait_immutable_references() {
    let param: Parameter = Parameter::init(
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

    let param: Parameter = Parameter::init(Similarity::Contains, complex_values.clone());

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
    assert_eq!(*name_param.similarity(), Similarity::Equals);
    assert_eq!(*name_param.values(), vec!["test"]);

    let desc_param = inner_map.get("description").unwrap();
    assert_eq!(*desc_param.similarity(), Similarity::Contains);
    assert_eq!(*desc_param.values(), vec!["rust"]);
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
        Parameter::init(
            Similarity::Between,
            vec!["10".to_string(), "20".to_string()],
        ),
    );

    // Verify the change
    assert_eq!(inner_map.len(), 2);
    assert!(inner_map.contains_key("new_key"));

    let new_param = inner_map.get("new_key").unwrap();
    assert_eq!(*new_param.similarity(), Similarity::Between);
    assert_eq!(*new_param.values(), vec!["10", "20"]);
}

#[test]
fn test_order_inner_method() {
    let mut order = Order::new();
    order.ascending("name".to_string());
    order.descending("date_created".to_string());

    // Test inner() method returns reference to IndexMap
    let inner_map = order.inner();
    assert_eq!(inner_map.len(), 2);
    assert!(inner_map.contains_key("name"));
    assert!(inner_map.contains_key("date_created"));

    // Test that we can access the underlying data
    assert_eq!(inner_map.get("name"), Some(&SortDirection::Ascending));
    assert_eq!(
        inner_map.get("date_created"),
        Some(&SortDirection::Descending)
    );
}

#[test]
fn test_order_inner_mut_method() {
    let mut order = Order::new();
    order.ascending("name".to_string());

    // Test inner_mut() method returns mutable reference
    let inner_map = order.inner_mut();
    assert_eq!(inner_map.len(), 1);

    // Test that we can modify the underlying data
    inner_map.insert("new_field".to_string(), SortDirection::Descending);

    // Verify the change
    assert_eq!(inner_map.len(), 2);
    assert!(inner_map.contains_key("new_field"));
    assert_eq!(inner_map.get("new_field"), Some(&SortDirection::Descending));
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
        Parameter::init(Similarity::Greater, vec!["5".to_string()]),
    );

    let keys_after: Vec<&String> = inner_map_mut.keys().collect();
    assert_eq!(keys_after, vec!["first", "second", "third", "fourth"]);
}

#[test]
fn test_inner_methods_with_empty_collections() {
    let params = Parameters::new();
    let order = Order::new();

    // Test inner() with empty collections
    assert_eq!(params.inner().len(), 0);
    assert_eq!(order.inner().len(), 0);

    // Test inner_mut() with empty collections
    let mut params_mut = Parameters::new();
    let mut order_mut = Order::new();

    assert_eq!(params_mut.inner_mut().len(), 0);
    assert_eq!(order_mut.inner_mut().len(), 0);
}

// ============================================================================
// METHOD RENAME TESTS (ascending/descending)
// ============================================================================

#[test]
fn test_order_ascending_method() {
    let mut order = Order::new();
    order.ascending("name".to_string());

    assert_eq!(order.inner().len(), 1);
    assert_eq!(order.inner().get("name"), Some(&SortDirection::Ascending));
}

#[test]
fn test_order_descending_method() {
    let mut order = Order::new();
    order.descending("date_created".to_string());

    assert_eq!(order.inner().len(), 1);
    assert_eq!(
        order.inner().get("date_created"),
        Some(&SortDirection::Descending)
    );
}

#[test]
fn test_order_ascending_descending_fluent_api() {
    let mut order = Order::new();
    order
        .ascending("name".to_string())
        .descending("date_created".to_string())
        .ascending("email".to_string());

    assert_eq!(order.inner().len(), 3);
    assert_eq!(order.inner().get("name"), Some(&SortDirection::Ascending));
    assert_eq!(
        order.inner().get("date_created"),
        Some(&SortDirection::Descending)
    );
    assert_eq!(order.inner().get("email"), Some(&SortDirection::Ascending));
}

#[test]
fn test_order_ascending_descending_overwrite() {
    let mut order = Order::new();
    order.ascending("name".to_string());
    order.descending("name".to_string()); // Should overwrite

    assert_eq!(order.inner().len(), 1);
    assert_eq!(order.inner().get("name"), Some(&SortDirection::Descending));
}

#[test]
fn test_order_ascending_descending_with_special_characters() {
    let mut order = Order::new();
    order.ascending("user_name_123".to_string());
    order.descending("date_created_at".to_string());

    assert_eq!(order.inner().len(), 2);
    assert_eq!(
        order.inner().get("user_name_123"),
        Some(&SortDirection::Ascending)
    );
    assert_eq!(
        order.inner().get("date_created_at"),
        Some(&SortDirection::Descending)
    );
}

#[test]
fn test_order_ascending_descending_with_unicode() {
    let mut order = Order::new();
    order.ascending("用户_姓名".to_string());
    order.descending("创建_日期".to_string());

    assert_eq!(order.inner().len(), 2);
    assert_eq!(
        order.inner().get("用户_姓名"),
        Some(&SortDirection::Ascending)
    );
    assert_eq!(
        order.inner().get("创建_日期"),
        Some(&SortDirection::Descending)
    );
}

#[test]
fn test_order_ascending_descending_empty_strings() {
    let mut order = Order::new();
    order.ascending("".to_string());
    order.descending("  ".to_string());

    assert_eq!(order.inner().len(), 2);
    assert_eq!(order.inner().get(""), Some(&SortDirection::Ascending));
    assert_eq!(order.inner().get("  "), Some(&SortDirection::Descending));
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
    query.order.ascending("name".to_string());
    query.order.descending("date_created".to_string());

    // Test using inner() for complex operations
    let param_map = query.parameters.inner();
    let sort_map = query.order.inner();

    // Verify we can iterate and access data
    let param_keys: Vec<&String> = param_map.keys().collect();
    let sort_keys: Vec<&String> = sort_map.keys().collect();

    assert_eq!(param_keys, vec!["name", "description"]);
    assert_eq!(sort_keys, vec!["name", "date_created"]);

    // Test using inner_mut() for modifications
    let param_map_mut = query.parameters.inner_mut();
    let sort_map_mut = query.order.inner_mut();

    param_map_mut.insert(
        "new_param".to_string(),
        Parameter::init(Similarity::Greater, vec!["100".to_string()]),
    );
    sort_map_mut.insert("new_sort".to_string(), SortDirection::Ascending);

    // Verify modifications
    assert_eq!(param_map_mut.len(), 3);
    assert_eq!(sort_map_mut.len(), 3);
    assert!(param_map_mut.contains_key("new_param"));
    assert!(sort_map_mut.contains_key("new_sort"));
}

#[test]
fn test_renamed_methods_with_query_building() {
    let mut params = Parameters::new();
    let mut order = Order::new();

    // Build using new method names
    params
        .equals("name".to_string(), vec!["john".to_string()])
        .contains("description".to_string(), vec!["rust".to_string()]);

    order
        .ascending("name".to_string())
        .descending("date_created".to_string());

    let query = Query::init(params, order, 25, 10);

    // Verify the query was built correctly
    assert_eq!(query.parameters.inner().len(), 2);
    assert_eq!(query.order.inner().len(), 2);
    assert_eq!(query.limit, 25);
    assert_eq!(query.offset, 10);

    // Verify parameter access using trait methods
    let name_param = query.parameters.inner().get("name").unwrap();
    assert_eq!(name_param.similarity(), &Similarity::Equals);
    assert_eq!(name_param.values(), &vec!["john".to_string()]);

    // Verify sort field access
    assert_eq!(
        query.order.inner().get("name"),
        Some(&SortDirection::Ascending)
    );
    assert_eq!(
        query.order.inner().get("date_created"),
        Some(&SortDirection::Descending)
    );
}

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

#[test]
fn test_backward_compatibility_with_new_features() {
    // Test that old tuple access still works alongside new features
    let mut params = Parameters::new();
    params.equals("name".to_string(), vec!["john".to_string()]);

    let param = params.inner().get("name").unwrap();

    // Old tuple access should still work
    assert_eq!(*param.similarity(), Similarity::Equals);
    assert_eq!(*param.values(), vec!["john".to_string()]);

    // New trait access should also work
    assert_eq!(param.similarity(), &Similarity::Equals);
    assert_eq!(param.values(), &vec!["john".to_string()]);

    // Both should return the same values
    assert_eq!(*param.similarity(), *param.similarity());
    assert_eq!(*param.values(), *param.values());
}

// ============================================================================
// NORMAL QUERY PARAMETER TESTS
// ============================================================================

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

#[test]
fn test_query_from_http_repeated_parameters() {
    let query = Query::from_http("name=ben&name=john&name=alice".to_string()).unwrap();

    assert_eq!(query.parameters.inner().len(), 1);

    assert!(query.parameters.inner().contains_key("name"));
    let param = &query.parameters.inner()["name"];
    assert_eq!(*param.similarity(), Similarity::Equals);
    assert_eq!(*param.values(), vec!["ben", "john", "alice"]);
}

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

#[test]
fn test_query_from_http_empty_normal_values() {
    let query = Query::from_http("name=&age=20&status=".to_string()).unwrap();

    assert_eq!(query.parameters.inner().len(), 1);

    assert!(query.parameters.inner().contains_key("age"));
    let param = &query.parameters.inner()["age"];
    assert_eq!(*param.similarity(), Similarity::Equals);
    assert_eq!(*param.values(), vec!["20"]);
}

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

    let sql = query.to_sql();
    assert_eq!(sql, "WHERE name IN (?, ?) AND age = ? LIMIT ? OFFSET ?");
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_sql_mixed_normal_and_similarity() {
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
        Parameter::init(Similarity::Contains, vec!["20".to_string()]),
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
    let values = query.to_values();

    // Should only contain limit and offset
    assert_eq!(values.len(), 2);
    assert_eq!(values[0], sql::Value::Integer(50)); // default limit
    assert_eq!(values[1], sql::Value::Integer(0)); // default offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_with_custom_limit_offset() {
    let mut query = Query::new();
    query.limit = 100;
    query.offset = 25;

    let values = query.to_values();

    assert_eq!(values.len(), 2);
    assert_eq!(values[0], sql::Value::Integer(100));
    assert_eq!(values[1], sql::Value::Integer(25));
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_equals_integer() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "age".to_string(),
        Parameter::init(Similarity::Equals, vec!["25".to_string()]),
    );

    let values = query.to_values();

    assert_eq!(values.len(), 3);
    assert_eq!(values[0], sql::Value::Integer(25));
    assert_eq!(values[1], sql::Value::Integer(50)); // limit
    assert_eq!(values[2], sql::Value::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_equals_multiple_integers() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "age".to_string(),
        Parameter::init(
            Similarity::Equals,
            vec!["25".to_string(), "30".to_string(), "35".to_string()],
        ),
    );

    let values = query.to_values();

    assert_eq!(values.len(), 5);
    assert_eq!(values[0], sql::Value::Integer(25));
    assert_eq!(values[1], sql::Value::Integer(30));
    assert_eq!(values[2], sql::Value::Integer(35));
    assert_eq!(values[3], sql::Value::Integer(50)); // limit
    assert_eq!(values[4], sql::Value::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_equals_real() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "price".to_string(),
        Parameter::init(Similarity::Equals, vec!["25.5".to_string()]),
    );

    let values = query.to_values();

    assert_eq!(values.len(), 3);
    assert_eq!(values[0], sql::Value::Real(25.5));
    assert_eq!(values[1], sql::Value::Integer(50)); // limit
    assert_eq!(values[2], sql::Value::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_equals_text() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "name".to_string(),
        Parameter::init(Similarity::Equals, vec!["john".to_string()]),
    );

    let values = query.to_values();

    assert_eq!(values.len(), 3);
    assert_eq!(values[0], sql::Value::Text("john".to_string()));
    assert_eq!(values[1], sql::Value::Integer(50)); // limit
    assert_eq!(values[2], sql::Value::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_equals_null() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "deleted_at".to_string(),
        Parameter::init(Similarity::Equals, vec![sql::NULL.to_string()]),
    );

    let values = query.to_values();

    assert_eq!(values.len(), 3);
    assert_eq!(values[0], sql::Value::Null);
    assert_eq!(values[1], sql::Value::Integer(50)); // limit
    assert_eq!(values[2], sql::Value::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_contains() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "name".to_string(),
        Parameter::init(Similarity::Contains, vec!["john".to_string()]),
    );

    let values = query.to_values();

    assert_eq!(values.len(), 3);
    assert_eq!(values[0], sql::Value::Text("%john%".to_string()));
    assert_eq!(values[1], sql::Value::Integer(50)); // limit
    assert_eq!(values[2], sql::Value::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_contains_multiple() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "name".to_string(),
        Parameter::init(
            Similarity::Contains,
            vec!["john".to_string(), "jane".to_string()],
        ),
    );

    let values = query.to_values();

    assert_eq!(values.len(), 4);
    assert_eq!(values[0], sql::Value::Text("%john%".to_string()));
    assert_eq!(values[1], sql::Value::Text("%jane%".to_string()));
    assert_eq!(values[2], sql::Value::Integer(50)); // limit
    assert_eq!(values[3], sql::Value::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_starts_with() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "name".to_string(),
        Parameter::init(Similarity::StartsWith, vec!["john".to_string()]),
    );

    let values = query.to_values();

    assert_eq!(values.len(), 3);
    assert_eq!(values[0], sql::Value::Text("john%".to_string()));
    assert_eq!(values[1], sql::Value::Integer(50)); // limit
    assert_eq!(values[2], sql::Value::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_ends_with() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "name".to_string(),
        Parameter::init(Similarity::EndsWith, vec!["son".to_string()]),
    );

    let values = query.to_values();

    assert_eq!(values.len(), 3);
    assert_eq!(values[0], sql::Value::Text("%son".to_string()));
    assert_eq!(values[1], sql::Value::Integer(50)); // limit
    assert_eq!(values[2], sql::Value::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_between() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "age".to_string(),
        Parameter::init(
            Similarity::Between,
            vec!["20".to_string(), "30".to_string()],
        ),
    );

    let values = query.to_values();

    assert_eq!(values.len(), 4);
    assert_eq!(values[0], sql::Value::Integer(20));
    assert_eq!(values[1], sql::Value::Integer(30));
    assert_eq!(values[2], sql::Value::Integer(50)); // limit
    assert_eq!(values[3], sql::Value::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_lesser() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "price".to_string(),
        Parameter::init(Similarity::Lesser, vec!["100".to_string()]),
    );

    let values = query.to_values();

    assert_eq!(values.len(), 3);
    assert_eq!(values[0], sql::Value::Integer(100));
    assert_eq!(values[1], sql::Value::Integer(50)); // limit
    assert_eq!(values[2], sql::Value::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_lesser_or_equal() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "price".to_string(),
        Parameter::init(Similarity::LesserOrEqual, vec!["100".to_string()]),
    );

    let values = query.to_values();

    assert_eq!(values.len(), 3);
    assert_eq!(values[0], sql::Value::Integer(100));
    assert_eq!(values[1], sql::Value::Integer(50)); // limit
    assert_eq!(values[2], sql::Value::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_greater() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "price".to_string(),
        Parameter::init(Similarity::Greater, vec!["50".to_string()]),
    );

    let values = query.to_values();

    assert_eq!(values.len(), 3);
    assert_eq!(values[0], sql::Value::Integer(50));
    assert_eq!(values[1], sql::Value::Integer(50)); // limit
    assert_eq!(values[2], sql::Value::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_greater_or_equal() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "price".to_string(),
        Parameter::init(Similarity::GreaterOrEqual, vec!["50".to_string()]),
    );

    let values = query.to_values();

    assert_eq!(values.len(), 3);
    assert_eq!(values[0], sql::Value::Integer(50));
    assert_eq!(values[1], sql::Value::Integer(50)); // limit
    assert_eq!(values[2], sql::Value::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_mixed_types() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "name".to_string(),
        Parameter::init(Similarity::Contains, vec!["john".to_string()]),
    );
    query.parameters.inner_mut().insert(
        "age".to_string(),
        Parameter::init(Similarity::Equals, vec!["25".to_string()]),
    );
    query.parameters.inner_mut().insert(
        "price".to_string(),
        Parameter::init(Similarity::Greater, vec!["100.5".to_string()]),
    );
    query.parameters.inner_mut().insert(
        "deleted_at".to_string(),
        Parameter::init(Similarity::Equals, vec![sql::NULL.to_string()]),
    );

    let values = query.to_values();

    assert_eq!(values.len(), 6);
    assert_eq!(values[0], sql::Value::Text("%john%".to_string()));
    assert_eq!(values[1], sql::Value::Integer(25));
    assert_eq!(values[2], sql::Value::Real(100.5));
    assert_eq!(values[3], sql::Value::Null);
    assert_eq!(values[4], sql::Value::Integer(50)); // limit
    assert_eq!(values[5], sql::Value::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_complex_numeric_types() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "integers".to_string(),
        Parameter::init(
            Similarity::Equals,
            vec!["123".to_string(), "-456".to_string(), "0".to_string()],
        ),
    );
    query.parameters.inner_mut().insert(
        "reals".to_string(),
        Parameter::init(
            Similarity::Equals,
            vec![
                "123.45".to_string(),
                "-456.78".to_string(),
                "0.0".to_string(),
            ],
        ),
    );
    query.parameters.inner_mut().insert(
        "mixed".to_string(),
        Parameter::init(
            Similarity::Equals,
            vec!["123".to_string(), "123.45".to_string(), "text".to_string()],
        ),
    );

    let values = query.to_values();

    assert_eq!(values.len(), 11);
    // integers
    assert_eq!(values[0], sql::Value::Integer(123));
    assert_eq!(values[1], sql::Value::Integer(-456));
    assert_eq!(values[2], sql::Value::Integer(0));
    // reals
    assert_eq!(values[3], sql::Value::Real(123.45));
    assert_eq!(values[4], sql::Value::Real(-456.78));
    assert_eq!(values[5], sql::Value::Real(0.0));
    // mixed
    assert_eq!(values[6], sql::Value::Integer(123));
    assert_eq!(values[7], sql::Value::Real(123.45));
    assert_eq!(values[8], sql::Value::Text("text".to_string()));
    // limit and offset
    assert_eq!(values[9], sql::Value::Integer(50));
    assert_eq!(values[10], sql::Value::Integer(0));
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_edge_case_strings() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "special_chars".to_string(),
        Parameter::init(
            Similarity::Contains,
            vec!["hello%world".to_string(), "test_underscore".to_string()],
        ),
    );
    query.parameters.inner_mut().insert(
        "unicode".to_string(),
        Parameter::init(
            Similarity::StartsWith,
            vec!["测试".to_string(), "héllo".to_string()],
        ),
    );
    query.parameters.inner_mut().insert(
        "empty_string".to_string(),
        Parameter::init(Similarity::Equals, vec!["".to_string()]),
    );

    let values = query.to_values();

    assert_eq!(values.len(), 6);
    assert_eq!(values[0], sql::Value::Text("%hello%world%".to_string()));
    assert_eq!(values[1], sql::Value::Text("%test_underscore%".to_string()));
    assert_eq!(values[2], sql::Value::Text("测试%".to_string()));
    assert_eq!(values[3], sql::Value::Text("héllo%".to_string()));
    assert_eq!(values[4], sql::Value::Integer(50)); // limit
    assert_eq!(values[5], sql::Value::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_numeric_edge_cases() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "edge_numbers".to_string(),
        Parameter::init(
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

    let values = query.to_values();

    assert_eq!(values.len(), 10);
    assert_eq!(values[0], sql::Value::Integer(0));
    assert_eq!(values[1], sql::Value::Integer(0));
    assert_eq!(values[2], sql::Value::Real(0.0));
    assert_eq!(values[3], sql::Value::Real(-0.0));
    assert_eq!(values[4], sql::Value::Real(1e10));
    assert_eq!(values[5], sql::Value::Real(1.5e-5));
    assert_eq!(values[6], sql::Value::Real(f64::INFINITY));
    // NaN doesn't equal NaN, so we need to check it's a Real with NaN
    match values[7] {
        sql::Value::Real(nan) => assert!(nan.is_nan()),
        _ => panic!("Expected Real(NaN), got {:?}", values[7]),
    }
    assert_eq!(values[8], sql::Value::Integer(50)); // limit
    assert_eq!(values[9], sql::Value::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_large_numbers() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "large_int".to_string(),
        Parameter::init(
            Similarity::Equals,
            vec![
                "9223372036854775807".to_string(),  // i64::MAX
                "-9223372036854775808".to_string(), // i64::MIN
            ],
        ),
    );
    query.parameters.inner_mut().insert(
        "large_real".to_string(),
        Parameter::init(
            Similarity::Equals,
            vec![
                "1.7976931348623157e308".to_string(),  // f64::MAX
                "-1.7976931348623157e308".to_string(), // f64::MIN
            ],
        ),
    );

    let values = query.to_values();

    assert_eq!(values.len(), 6);
    assert_eq!(values[0], sql::Value::Integer(9223372036854775807));
    assert_eq!(values[1], sql::Value::Integer(-9223372036854775808));
    assert_eq!(values[2], sql::Value::Real(1.7976931348623157e308));
    assert_eq!(values[3], sql::Value::Real(-1.7976931348623157e308));
    assert_eq!(values[4], sql::Value::Integer(50)); // limit
    assert_eq!(values[5], sql::Value::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_null_variations() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "null_tests".to_string(),
        Parameter::init(
            Similarity::Equals,
            vec![
                sql::NULL.to_string(),
                "NULL".to_string(),
                "Null".to_string(),
                "nUlL".to_string(),
            ],
        ),
    );

    let values = query.to_values();

    assert_eq!(values.len(), 6);
    assert_eq!(values[0], sql::Value::Null);
    assert_eq!(values[1], sql::Value::Text("NULL".to_string()));
    assert_eq!(values[2], sql::Value::Text("Null".to_string()));
    assert_eq!(values[3], sql::Value::Text("nUlL".to_string()));
    assert_eq!(values[4], sql::Value::Integer(50)); // limit
    assert_eq!(values[5], sql::Value::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_whitespace_handling() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "whitespace".to_string(),
        Parameter::init(
            Similarity::Equals,
            vec![
                " 123 ".to_string(),
                " 123.45 ".to_string(),
                " hello ".to_string(),
            ],
        ),
    );

    let values = query.to_values();

    assert_eq!(values.len(), 5);
    assert_eq!(values[0], sql::Value::Text(" 123 ".to_string()));
    assert_eq!(values[1], sql::Value::Text(" 123.45 ".to_string()));
    assert_eq!(values[2], sql::Value::Text(" hello ".to_string()));
    assert_eq!(values[3], sql::Value::Integer(50)); // limit
    assert_eq!(values[4], sql::Value::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_empty_parameters() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "empty".to_string(),
        Parameter::init(Similarity::Equals, vec![]),
    );

    let values = query.to_values();

    // Should only contain limit and offset
    assert_eq!(values.len(), 2);
    assert_eq!(values[0], sql::Value::Integer(50)); // limit
    assert_eq!(values[1], sql::Value::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_multiple_parameters_same_key() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "name".to_string(),
        Parameter::init(Similarity::Contains, vec!["john".to_string()]),
    );
    query.parameters.inner_mut().insert(
        "name".to_string(), // This should overwrite the previous
        Parameter::init(Similarity::StartsWith, vec!["jane".to_string()]),
    );

    let values = query.to_values();

    assert_eq!(values.len(), 3);
    assert_eq!(values[0], sql::Value::Text("jane%".to_string()));
    assert_eq!(values[1], sql::Value::Integer(50)); // limit
    assert_eq!(values[2], sql::Value::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_parameter_order_preservation() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "first".to_string(),
        Parameter::init(Similarity::Equals, vec!["1".to_string()]),
    );
    query.parameters.inner_mut().insert(
        "second".to_string(),
        Parameter::init(Similarity::Contains, vec!["2".to_string()]),
    );
    query.parameters.inner_mut().insert(
        "third".to_string(),
        Parameter::init(Similarity::Between, vec!["3".to_string(), "4".to_string()]),
    );

    let values = query.to_values();

    assert_eq!(values.len(), 6);
    assert_eq!(values[0], sql::Value::Integer(1));
    assert_eq!(values[1], sql::Value::Text("%2%".to_string()));
    assert_eq!(values[2], sql::Value::Integer(3));
    assert_eq!(values[3], sql::Value::Integer(4));
    assert_eq!(values[4], sql::Value::Integer(50)); // limit
    assert_eq!(values[5], sql::Value::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_with_custom_limit_offset_complex() {
    let mut query = Query::new();
    query.limit = 100;
    query.offset = 25;

    query.parameters.inner_mut().insert(
        "name".to_string(),
        Parameter::init(Similarity::Contains, vec!["john".to_string()]),
    );
    query.parameters.inner_mut().insert(
        "age".to_string(),
        Parameter::init(
            Similarity::Between,
            vec!["20".to_string(), "30".to_string()],
        ),
    );

    let values = query.to_values();

    assert_eq!(values.len(), 5);
    assert_eq!(values[0], sql::Value::Text("%john%".to_string()));
    assert_eq!(values[1], sql::Value::Integer(20));
    assert_eq!(values[2], sql::Value::Integer(30));
    assert_eq!(values[3], sql::Value::Integer(100)); // custom limit
    assert_eq!(values[4], sql::Value::Integer(25)); // custom offset
}

#[cfg(feature = "sql")]
#[test]
fn test_sql_value_enum_variants() {
    // Test all SQLValue enum variants
    let null = sql::Value::Null;
    let integer = sql::Value::Integer(42);
    let real = sql::Value::Real(3.14);
    let text = sql::Value::Text("hello".to_string());
    let blob = sql::Value::Blob(vec![1, 2, 3, 4]);

    assert_eq!(null, sql::Value::Null);
    assert_eq!(integer, sql::Value::Integer(42));
    assert_eq!(real, sql::Value::Real(3.14));
    assert_eq!(text, sql::Value::Text("hello".to_string()));
    assert_eq!(blob, sql::Value::Blob(vec![1, 2, 3, 4]));
}

#[cfg(feature = "sql")]
#[test]
fn test_sql_value_enum_derived_traits() {
    // Test Clone
    let original = sql::Value::Text("test".to_string());
    let cloned = original.clone();
    assert_eq!(original, cloned);

    // Test Debug
    let debug_str = format!("{:?}", sql::Value::Integer(42));
    assert!(debug_str.contains("Integer"));
    assert!(debug_str.contains("42"));

    // Test PartialEq
    assert_eq!(sql::Value::Null, sql::Value::Null);
    assert_eq!(sql::Value::Integer(42), sql::Value::Integer(42));
    assert_eq!(sql::Value::Real(3.14), sql::Value::Real(3.14));
    assert_eq!(
        sql::Value::Text("hello".to_string()),
        sql::Value::Text("hello".to_string())
    );
    assert_eq!(
        sql::Value::Blob(vec![1, 2, 3]),
        sql::Value::Blob(vec![1, 2, 3])
    );

    // Test inequality
    assert_ne!(sql::Value::Null, sql::Value::Integer(0));
    assert_ne!(sql::Value::Integer(42), sql::Value::Integer(43));
    assert_ne!(sql::Value::Real(3.14), sql::Value::Real(3.15));
    assert_ne!(
        sql::Value::Text("hello".to_string()),
        sql::Value::Text("world".to_string())
    );
    assert_ne!(
        sql::Value::Blob(vec![1, 2, 3]),
        sql::Value::Blob(vec![1, 2, 4])
    );
}

#[cfg(feature = "sql")]
#[test]
fn test_query_parameter_values() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "name".to_string(),
        Parameter::init(Similarity::Contains, vec!["john".to_string()]),
    );
    query.parameters.inner_mut().insert(
        "age".to_string(),
        Parameter::init(Similarity::Equals, vec!["25".to_string()]),
    );

    let values = query.parameter_values();

    assert_eq!(values.len(), 2);
    assert_eq!(values[0], sql::Value::Text("%john%".to_string()));
    assert_eq!(values[1], sql::Value::Integer(25));
}

#[cfg(feature = "sql")]
#[test]
fn test_query_parameter_values_empty() {
    let query = Query::new();
    let values = query.parameter_values();

    assert_eq!(values.len(), 0);
}

#[cfg(feature = "sql")]
#[test]
fn test_query_pagination_values() {
    let mut query = Query::new();
    query.limit = 100;
    query.offset = 25;

    let values = query.pagination_values();

    assert_eq!(values.len(), 2);
    assert_eq!(values[0], sql::Value::Integer(100));
    assert_eq!(values[1], sql::Value::Integer(25));
}

#[cfg(feature = "sql")]
#[test]
fn test_query_total_parameters() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "name".to_string(),
        Parameter::init(
            Similarity::Contains,
            vec!["john".to_string(), "jane".to_string()],
        ),
    );
    query.parameters.inner_mut().insert(
        "age".to_string(),
        Parameter::init(Similarity::Equals, vec!["25".to_string()]),
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
    query.parameters.inner_mut().insert(
        "empty".to_string(),
        Parameter::init(Similarity::Equals, vec![]),
    );

    let count = query.total_parameters();

    // No parameter values + 2 pagination = 2 total
    assert_eq!(count, 2);
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_ignores_empty_values() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "name".to_string(),
        Parameter::init(
            Similarity::Contains,
            vec![
                "john".to_string(),
                "".to_string(), // empty string
                "jane".to_string(),
                "   ".to_string(), // whitespace only
                "bob".to_string(),
            ],
        ),
    );
    query.parameters.inner_mut().insert(
        "age".to_string(),
        Parameter::init(
            Similarity::Equals,
            vec![
                "25".to_string(),
                "".to_string(), // empty string
                "30".to_string(),
            ],
        ),
    );

    let values = query.to_values();

    // Should only contain non-empty values: john, jane, bob, 25, 30 + limit + offset = 7
    assert_eq!(values.len(), 7);
    assert_eq!(values[0], sql::Value::Text("%john%".to_string()));
    assert_eq!(values[1], sql::Value::Text("%jane%".to_string()));
    assert_eq!(values[2], sql::Value::Text("%bob%".to_string()));
    assert_eq!(values[3], sql::Value::Integer(25));
    assert_eq!(values[4], sql::Value::Integer(30));
    assert_eq!(values[5], sql::Value::Integer(50)); // limit
    assert_eq!(values[6], sql::Value::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_parameter_values_ignores_empty_values() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "name".to_string(),
        Parameter::init(
            Similarity::Contains,
            vec![
                "john".to_string(),
                "".to_string(), // empty string
                "jane".to_string(),
                "   ".to_string(), // whitespace only
            ],
        ),
    );

    let values = query.parameter_values();

    // Should only contain non-empty values: john, jane
    assert_eq!(values.len(), 2);
    assert_eq!(values[0], sql::Value::Text("%john%".to_string()));
    assert_eq!(values[1], sql::Value::Text("%jane%".to_string()));
}

#[cfg(feature = "sql")]
#[test]
fn test_query_total_parameters_ignores_empty_values() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "name".to_string(),
        Parameter::init(
            Similarity::Contains,
            vec![
                "john".to_string(),
                "".to_string(), // empty string
                "jane".to_string(),
                "   ".to_string(), // whitespace only
            ],
        ),
    );
    query.parameters.inner_mut().insert(
        "age".to_string(),
        Parameter::init(
            Similarity::Equals,
            vec![
                "25".to_string(),
                "".to_string(), // empty string
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
    query.parameters.inner_mut().insert(
        "name".to_string(),
        Parameter::init(
            Similarity::Contains,
            vec![
                "".to_string(),    // empty string
                "   ".to_string(), // whitespace only
                "\t".to_string(),  // tab only
                "\n".to_string(),  // newline only
            ],
        ),
    );

    let values = query.to_values();

    // Should only contain limit and offset (no parameter values)
    assert_eq!(values.len(), 2);
    assert_eq!(values[0], sql::Value::Integer(50)); // limit
    assert_eq!(values[1], sql::Value::Integer(0)); // offset
}

#[cfg(feature = "sql")]
#[test]
fn test_query_to_values_mixed_empty_and_null() {
    let mut query = Query::new();
    query.parameters.inner_mut().insert(
        "status".to_string(),
        Parameter::init(
            Similarity::Equals,
            vec![
                "".to_string(),        // empty string (should be ignored)
                sql::NULL.to_string(), // null string (should be converted to SQLValue::Null)
                "   ".to_string(),     // whitespace only (should be ignored)
                "active".to_string(),  // normal value
            ],
        ),
    );

    let values = query.to_values();

    // Should contain: null, active + limit + offset = 4 total
    assert_eq!(values.len(), 4);
    assert_eq!(values[0], sql::Value::Null);
    assert_eq!(values[1], sql::Value::Text("active".to_string()));
    assert_eq!(values[2], sql::Value::Integer(50)); // limit
    assert_eq!(values[3], sql::Value::Integer(0)); // offset
}
