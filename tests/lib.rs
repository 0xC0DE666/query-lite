#[cfg(test)]
use http_to_sql::Query;

#[test]
fn to_http_empty_params() {
    let qry = Query::init(10, 0);
    let result = qry.to_http();
    let expected = "order=date_created:desc&limit=10&offset=0&".to_string();
    assert_eq!(result, expected);
}

#[test]
fn to_http_single_param() {
    let mut qry = Query::init(20, 5);
    qry.params
        .insert("name".to_string(), vec!["alice".to_string()]);
    let result = qry.to_http();
    let expected = "order=date_created:desc&name=alice&limit=20&offset=5&".to_string();
    assert_eq!(result, expected);
}

#[test]
fn to_http_multiple_values() {
    let mut qry = Query::init(50, 10);
    qry.params.insert(
        "name".to_string(),
        vec!["alice".to_string(), "bob".to_string()],
    );
    qry.params
        .insert("age".to_string(), vec!["30".to_string(), "40".to_string()]);
    let result = qry.to_http();
    let expected =
        "order=date_created:desc&name=alice,bob&age=30,40&limit=50&offset=10&".to_string();
    assert_eq!(result, expected);
}

#[test]
fn to_http_ignores_empty_values() {
    let mut qry = Query::init(5, 2);
    qry.params.insert("name".to_string(), vec![]);
    qry.params.insert("age".to_string(), vec!["25".to_string()]);
    qry.params
        .insert("date_created".to_string(), vec!["123-456".to_string()]);
    let result = qry.to_http();
    let expected =
        "order=date_created:desc&age=25&date_created=123-456&limit=5&offset=2&".to_string();
    assert_eq!(result, expected);
}

#[test]
fn from_http_basic() {
    let qry = "age=25&name=john,sarah,james&limit=5&offset=2".to_string();
    let result = Query::from_http(qry);

    let mut expected = Query::init(5, 2);
    expected
        .params
        .insert("age".to_string(), vec!["25".to_string()]);
    expected.params.insert(
        "name".to_string(),
        vec!["john".to_string(), "sarah".to_string(), "james".to_string()],
    );

    assert_eq!(result.params, expected.params);
    assert_eq!(result.limit, expected.limit);
    assert_eq!(result.offset, expected.offset);
}

#[test]
fn from_http_with_ranges() {
    let qry = "balance=10.0-100.0&date_created=123-456&limit=10&offset=0".to_string();
    let result = Query::from_http(qry);

    let mut expected = Query::init(10, 0);
    expected
        .params
        .insert("balance".to_string(), vec!["10.0-100.0".to_string()]);
    expected
        .params
        .insert("date_created".to_string(), vec!["123-456".to_string()]);

    assert_eq!(result.params, expected.params);
    assert_eq!(result.limit, expected.limit);
    assert_eq!(result.offset, expected.offset);
}

#[test]
fn from_http_with_null_value() {
    let qry = "limit=1&offset=1".to_string();
    let result = Query::from_http(qry);
    let expected = Query::init(1, 1);

    assert_eq!(result.params, expected.params);
    assert_eq!(result.limit, expected.limit);
    assert_eq!(result.offset, expected.offset);
}

#[test]
fn from_http_empty_qry() {
    let qry = "limit=0&offset=0".to_string();
    let result = Query::from_http(qry);

    let expected = Query::init(0, 0);

    assert_eq!(result.params, expected.params);
    assert_eq!(result.limit, expected.limit);
    assert_eq!(result.offset, expected.offset);
}

#[test]
fn from_http_missing_limit_offset() {
    let qry = "name=alice,bob&age=30".to_string();
    let result = Query::from_http(qry);

    let mut expected = Query::init(40, 0);
    expected.params.insert(
        "name".to_string(),
        vec!["alice".to_string(), "bob".to_string()],
    );
    expected
        .params
        .insert("age".to_string(), vec!["30".to_string()]);

    assert_eq!(result.params, expected.params);
    assert_eq!(result.limit, expected.limit);
    assert_eq!(result.offset, expected.offset);
}

fn init_query() -> Query {
    let mut query = Query::new();
    query.params.insert("a".to_string(), vec!["1".to_string()]);
    query.params.insert("b".to_string(), vec!["2".to_string()]);
    query.params.insert("c".to_string(), vec!["3".to_string()]);
    query
}

#[test]
fn keep_empty_keys() {
    let params = init_query();
    let result = params.keep(vec![]);
    assert_eq!(
        result.params.len(),
        0,
        "Should keep no keys if input is empty"
    );
}

#[test]
fn keep_all_keys() {
    let params = init_query();
    let result = params.keep(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    assert_eq!(
        result.params.len(),
        3,
        "Should keep all keys if all are listed"
    );
    assert_eq!(result.params.get("a"), Some(&vec!["1".to_string()]));
    assert_eq!(result.params.get("b"), Some(&vec!["2".to_string()]));
    assert_eq!(result.params.get("c"), Some(&vec!["3".to_string()]));
}

#[test]
fn keep_some_keys() {
    let params = init_query();
    let result = params.keep(vec!["a".to_string(), "c".to_string()]);
    assert_eq!(result.params.len(), 2, "Should keep only specified keys");
    assert_eq!(result.params.get("a"), Some(&vec!["1".to_string()]));
    assert_eq!(result.params.get("c"), Some(&vec!["3".to_string()]));
    assert_eq!(result.params.get("b"), None, "Should remove unlisted key");
}

#[test]
fn keep_nonexistent_keys() {
    let params = init_query();
    let result = params.keep(vec!["x".to_string(), "y".to_string()]);
    assert_eq!(
        result.params.len(),
        0,
        "Should keep nothing if no keys match"
    );
}

#[test]
fn keep_empty_params() {
    let params = Query::new();
    let result = params.keep(vec!["a".to_string()]);
    assert_eq!(result.params.len(), 0, "Should handle empty params");
}

#[test]
fn remove_empty_keys() {
    let params = init_query();
    let result = params.remove(vec![]);
    assert_eq!(
        result.params.len(),
        3,
        "Should remove nothing if input is empty"
    );
    assert_eq!(result.params.get("a"), Some(&vec!["1".to_string()]));
    assert_eq!(result.params.get("b"), Some(&vec!["2".to_string()]));
    assert_eq!(result.params.get("c"), Some(&vec!["3".to_string()]));
}

#[test]
fn remove_all_keys() {
    let params = init_query();
    let result = params.remove(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    assert_eq!(
        result.params.len(),
        0,
        "Should remove all keys if all are listed"
    );
}

#[test]
fn remove_some_keys() {
    let params = init_query();
    let result = params.remove(vec!["a".to_string(), "c".to_string()]);
    assert_eq!(result.params.len(), 1, "Should remove only specified keys");
    assert_eq!(result.params.get("a"), None);
    assert_eq!(result.params.get("c"), None);
    assert_eq!(
        result.params.get("b"),
        Some(&vec!["2".to_string()]),
        "Should keep unlisted key"
    );
}

#[test]
fn remove_nonexistent_keys() {
    let params = init_query();
    let result = params.remove(vec!["x".to_string(), "y".to_string()]);
    assert_eq!(
        result.params.len(),
        3,
        "Should remove nothing if no keys match"
    );
    assert_eq!(result.params.get("a"), Some(&vec!["1".to_string()]));
    assert_eq!(result.params.get("b"), Some(&vec!["2".to_string()]));
    assert_eq!(result.params.get("c"), Some(&vec!["3".to_string()]));
}

#[test]
fn remove_empty_params() {
    let params = Query::new();
    let result = params.remove(vec!["a".to_string()]);
    assert_eq!(result.params.len(), 0, "Should handle empty params");
}
