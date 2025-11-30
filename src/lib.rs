pub mod error;

use error::{Error, Result};
use indexmap::IndexMap;
use std::str::FromStr;
use url::form_urlencoded;

// Main types
#[derive(Clone, Debug, PartialEq)]
pub struct Query {
    pub parameters: Parameters,
    pub order: Order,
    pub limit: usize,
    pub offset: usize,
}

impl Query {
    pub fn new() -> Self {
        Self {
            parameters: Parameters::new(),
            order: Order::new(),
            limit: Parameters::DEFAULT_LIMIT,
            offset: Parameters::DEFAULT_OFFSET,
        }
    }

    pub fn init(parameters: Parameters, order: Order, limit: usize, offset: usize) -> Self {
        Self {
            parameters,
            order,
            limit,
            offset,
        }
    }

    pub fn to_http(&self) -> String {
        let mut params_str = self
            .parameters
            .inner()
            .iter()
            .filter(|(_, param)| param.values().len() > 0)
            .map(|(key, param)| {
                let similarity_str = param.similarity().to_string();
                let values_str = param
                    .values()
                    .iter()
                    .map(|v| url_encode(v))
                    .collect::<Vec<String>>()
                    .join(&format!("{COMMA}"));
                format!("{key}{EQUAL}{similarity_str}{COLON}{values_str}",)
            })
            .collect::<Vec<String>>()
            .join("&");

        let order_str = self
            .order
            .inner()
            .iter()
            .filter(|(name, _)| name.len() > 0)
            .map(|(name, sort_order)| format!("{name}{COLON}{}", sort_order.to_string()))
            .collect::<Vec<String>>()
            .join(&format!("{COMMA}"));

        if params_str.len() > 0 {
            params_str.push_str(&format!("{AMPERSAND}"));
        }

        if order_str.len() > 0 {
            params_str.push_str(&format!("{}{EQUAL}{}", Parameters::ORDER, order_str));
            params_str.push_str(&format!("{AMPERSAND}"));
        }

        format!(
            "{params_str}{}{EQUAL}{}{AMPERSAND}{}{EQUAL}{}",
            Parameters::LIMIT,
            self.limit,
            Parameters::OFFSET,
            self.offset,
        )
    }

    // name=contains:damian&surname=equals:black,steel,wood&order=date_created:desc&limit=40&offset=0
    pub fn from_http(search: String) -> Result<Self> {
        let mut query = Self::new();
        let trimmed_search = search.trim_start_matches(QUESTION).trim();

        if trimmed_search.is_empty() {
            return Ok(query);
        }

        for k_v in trimmed_search.split(AMPERSAND) {
            let trimmed_kv = k_v.trim();
            if trimmed_kv.is_empty() {
                continue;
            }

            let mut parts = trimmed_kv.splitn(2, EQUAL);
            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                let trimmed_key = key.trim();
                let trimmed_value = value.trim();

                if trimmed_key.is_empty() || trimmed_value.is_empty() {
                    continue;
                }

                match trimmed_key {
                    Parameters::ORDER => {
                        // Check if the value looks like a sort field format (contains colon)
                        if !trimmed_value.contains(COLON) {
                            // Fail on clearly invalid formats (like "invalid")
                            return Err(Error::InvalidSortField(trimmed_value.into()));
                        }

                        if let Ok(order) = Order::from_str(trimmed_value) {
                            query.order = order;
                        }
                        // Skip malformed sort fields (like ":desc")
                    }
                    Parameters::LIMIT => {
                        query.limit = trimmed_value.parse().unwrap_or(Parameters::DEFAULT_LIMIT);
                    }
                    Parameters::OFFSET => {
                        query.offset = trimmed_value.parse().unwrap_or(Parameters::DEFAULT_OFFSET);
                    }
                    _k => {
                        // Check if this is a similarity-based parameter (contains colon)
                        if trimmed_value.contains(COLON) {
                            // Parse as similarity-based parameter
                            let Parameter(similarity, values) = parse_parameter(trimmed_value)?;
                            // Only add parameters that have values
                            if values.is_empty() {
                                continue;
                            }
                            // Replace any existing parameter (similarity-based takes precedence)
                            query
                                .parameters
                                .0
                                .insert(trimmed_key.to_string(), Parameter(similarity, values));
                        } else {
                            // Handle as normal query parameter (default to equals similarity)
                            let decoded_value = url_decode(trimmed_value);

                            // Check if parameter already exists and is not similarity-based
                            if let Some(existing_param) =
                                query.parameters.0.get_mut(&trimmed_key.to_string())
                            {
                                // Only append if the existing parameter is also equals similarity
                                if *existing_param.similarity() == Similarity::Equals {
                                    existing_param.1.push(decoded_value);
                                }
                                // If existing parameter is similarity-based, ignore this normal parameter
                            } else {
                                // Create new parameter with equals similarity
                                query.parameters.0.insert(
                                    trimmed_key.to_string(),
                                    Parameter::init(Similarity::Equals, vec![decoded_value]),
                                );
                            }
                        }
                    }
                }
            } else {
                return Err(Error::InvalidSearchParameters(search));
            }
        }

        Ok(query)
    }

    #[cfg(feature = "sql")]
    pub fn to_sql(&self) -> String {
        let mut sql_parts = Vec::new();

        // Build WHERE clause from parameters
        if let Some(where_clause) = self.where_clause() {
            sql_parts.push(format!("WHERE {}", where_clause));
        }

        // Build ORDER BY clause from order
        if let Some(order_clause) = self.order_clause() {
            sql_parts.push(format!("ORDER BY {}", order_clause));
        }

        // Add LIMIT and OFFSET
        sql_parts.push(format!("LIMIT ? OFFSET ?"));

        sql_parts.join(" ")
    }

    #[cfg(feature = "sql")]
    pub fn where_clause(&self) -> Option<String> {
        let mut conditions = Vec::new();

        for (key, param) in &self.parameters.0 {
            let similarity = param.similarity();
            let values = param.values();
            if values.is_empty() {
                continue;
            }

            let condition = match similarity {
                Similarity::Equals => {
                    if values.len() == 1 {
                        if values[0] == "null" {
                            format!("{} IS ?", key)
                        } else {
                            format!("{} = ?", key)
                        }
                    } else {
                        let placeholders = vec!["?"; values.len()].join(", ");
                        format!("{} IN ({})", key, placeholders)
                    }
                }
                Similarity::Contains => {
                    if values.len() == 1 {
                        format!("{} LIKE ?", key)
                    } else {
                        let like_conditions: Vec<String> =
                            values.iter().map(|_| format!("{} LIKE ?", key)).collect();
                        format!("({})", like_conditions.join(" OR "))
                    }
                }
                Similarity::StartsWith => {
                    if values.len() == 1 {
                        format!("{} LIKE ?", key)
                    } else {
                        let like_conditions: Vec<String> =
                            values.iter().map(|_| format!("{} LIKE ?", key)).collect();
                        format!("({})", like_conditions.join(" OR "))
                    }
                }
                Similarity::EndsWith => {
                    if values.len() == 1 {
                        format!("{} LIKE ?", key)
                    } else {
                        let like_conditions: Vec<String> =
                            values.iter().map(|_| format!("{} LIKE ?", key)).collect();
                        format!("({})", like_conditions.join(" OR "))
                    }
                }
                Similarity::Between => {
                    if values.len() >= 2 {
                        // Group values into pairs, ignoring any odd value
                        let pairs: Vec<&[String]> = values.chunks(2).collect();
                        let between_conditions: Vec<String> = pairs
                            .iter()
                            .map(|pair| {
                                if pair.len() == 2 {
                                    format!("{} BETWEEN ? AND ?", key)
                                } else {
                                    String::new() // Skip incomplete pairs
                                }
                            })
                            .filter(|condition| !condition.is_empty())
                            .collect();

                        if between_conditions.is_empty() {
                            continue; // Skip if no valid pairs
                        } else if between_conditions.len() == 1 {
                            between_conditions[0].clone()
                        } else {
                            format!("({})", between_conditions.join(" OR "))
                        }
                    } else {
                        continue; // Skip invalid between conditions
                    }
                }
                Similarity::Lesser => {
                    if values.len() == 1 {
                        format!("{} < ?", key)
                    } else {
                        let conditions: Vec<String> =
                            values.iter().map(|_| format!("{} < ?", key)).collect();
                        format!("({})", conditions.join(" OR "))
                    }
                }
                Similarity::LesserOrEqual => {
                    if values.len() == 1 {
                        format!("{} <= ?", key)
                    } else {
                        let conditions: Vec<String> =
                            values.iter().map(|_| format!("{} <= ?", key)).collect();
                        format!("({})", conditions.join(" OR "))
                    }
                }
                Similarity::Greater => {
                    if values.len() == 1 {
                        format!("{} > ?", key)
                    } else {
                        let conditions: Vec<String> =
                            values.iter().map(|_| format!("{} > ?", key)).collect();
                        format!("({})", conditions.join(" OR "))
                    }
                }
                Similarity::GreaterOrEqual => {
                    if values.len() == 1 {
                        format!("{} >= ?", key)
                    } else {
                        let conditions: Vec<String> =
                            values.iter().map(|_| format!("{} >= ?", key)).collect();
                        format!("({})", conditions.join(" OR "))
                    }
                }
            };

            conditions.push(condition);
        }

        if conditions.is_empty() {
            None
        } else {
            Some(conditions.join(" AND "))
        }
    }

    #[cfg(feature = "sql")]
    pub fn order_clause(&self) -> Option<String> {
        let mut order_parts = Vec::new();

        for (name, sort_order) in &self.order.0 {
            if !name.is_empty() {
                let direction = match sort_order {
                    SortOrder::Ascending => "ASC",
                    SortOrder::Descending => "DESC",
                };
                order_parts.push(format!("{} {}", name, direction));
            }
        }

        if order_parts.is_empty() {
            None
        } else {
            Some(order_parts.join(", "))
        }
    }

    #[cfg(feature = "sql")]
    pub fn to_values(&self) -> Vec<SqlValue> {
        let mut sql_values = self.parameter_values();
        sql_values.extend(self.pagination_values());
        sql_values
    }

    #[cfg(feature = "sql")]
    /// Get SQL values for parameters only (without limit and offset)
    pub fn parameter_values(&self) -> Vec<SqlValue> {
        let mut sql_values = Vec::new();

        for (_k, param) in self.parameters.inner() {
            let param_similarity = param.similarity();
            let param_values = param.values();
            for cur_val in param_values {
                // Skip empty values
                if cur_val.trim().is_empty() {
                    continue;
                }

                if cur_val == "null" {
                    sql_values.push(SqlValue::Null);
                    continue;
                }

                let sql_value = match *param_similarity {
                    Similarity::Contains => SqlValue::Text(format!("%{}%", cur_val)),
                    Similarity::StartsWith => SqlValue::Text(format!("{}%", cur_val)),
                    Similarity::EndsWith => SqlValue::Text(format!("%{}", cur_val)),
                    _ => {
                        // Try to parse as integer first, then float, then text
                        if let Ok(i) = cur_val.parse::<i64>() {
                            SqlValue::Integer(i)
                        } else if let Ok(f) = cur_val.parse::<f64>() {
                            SqlValue::Real(f)
                        } else {
                            SqlValue::Text(cur_val.clone())
                        }
                    }
                };

                sql_values.push(sql_value);
            }
        }

        sql_values
    }

    #[cfg(feature = "sql")]
    /// Get SQL values for pagination (limit and offset only)
    pub fn pagination_values(&self) -> Vec<SqlValue> {
        vec![
            SqlValue::Integer(self.limit as i64),
            SqlValue::Integer(self.offset as i64),
        ]
    }

    #[cfg(feature = "sql")]
    /// Get the total number of SQL parameter values (parameters + pagination)
    /// This counts only non-empty values, matching the behavior of to_values()
    pub fn total_parameters(&self) -> usize {
        let parameter_count: usize = self
            .parameters
            .inner()
            .values()
            .map(|param| {
                param
                    .values()
                    .iter()
                    .filter(|v| !v.trim().is_empty())
                    .count()
            })
            .sum();

        parameter_count + 2 // +2 for limit and offset
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Parameters(IndexMap<String, Parameter>);

impl Parameters {
    pub const ORDER: &str = "order";
    pub const LIMIT: &str = "limit";
    pub const OFFSET: &str = "offset";

    pub const EXCLUDE: [&str; 3] = [Parameters::ORDER, Parameters::LIMIT, Parameters::OFFSET];

    pub const DEFAULT_LIMIT: usize = 50;
    pub const DEFAULT_OFFSET: usize = 0;

    pub fn new() -> Self {
        Self(IndexMap::new())
    }

    pub fn inner(&self) -> &IndexMap<String, Parameter> {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut IndexMap<String, Parameter> {
        &mut self.0
    }

    pub fn equals(&mut self, key: String, values: Vec<String>) -> &mut Self {
        self.0
            .insert(key, Parameter::init(Similarity::Equals, values));
        self
    }

    pub fn contains(&mut self, key: String, values: Vec<String>) -> &mut Self {
        self.0
            .insert(key, Parameter::init(Similarity::Contains, values));
        self
    }

    pub fn starts_with(&mut self, key: String, values: Vec<String>) -> &mut Self {
        self.0
            .insert(key, Parameter::init(Similarity::StartsWith, values));
        self
    }

    pub fn ends_with(&mut self, key: String, values: Vec<String>) -> &mut Self {
        self.0
            .insert(key, Parameter::init(Similarity::EndsWith, values));
        self
    }

    pub fn between(&mut self, key: String, values: Vec<String>) -> &mut Self {
        self.0
            .insert(key, Parameter::init(Similarity::Between, values));
        self
    }

    pub fn lesser(&mut self, key: String, values: Vec<String>) -> &mut Self {
        self.0
            .insert(key, Parameter::init(Similarity::Lesser, values));
        self
    }

    pub fn lesser_or_equal(&mut self, key: String, values: Vec<String>) -> &mut Self {
        self.0
            .insert(key, Parameter::init(Similarity::LesserOrEqual, values));
        self
    }

    pub fn greater(&mut self, key: String, values: Vec<String>) -> &mut Self {
        self.0
            .insert(key, Parameter::init(Similarity::Greater, values));
        self
    }

    pub fn greater_or_equal(&mut self, key: String, values: Vec<String>) -> &mut Self {
        self.0
            .insert(key, Parameter::init(Similarity::GreaterOrEqual, values));
        self
    }

    pub fn keep(&self, keys: Vec<String>) -> Self {
        let mut result = Self::new();
        for key in keys {
            if let Some(value) = self.0.get(&key) {
                result.0.insert(key, value.clone());
            }
        }
        result
    }

    pub fn remove(&self, keys: Vec<String>) -> Self {
        let mut result = self.clone();
        for key in keys {
            result.0.shift_remove(&key);
        }
        result
    }
}

impl Default for Parameters {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for Parameters {
    type Err = Error;

    // EXAMPLE INPUT
    // name=contains:damian&surname=equals:black,steel,wood&order=date_created:desc&limit=40&offset=0
    fn from_str(s: &str) -> Result<Self> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Ok(Parameters::new());
        }

        let str_parameters: Vec<&str> = trimmed.split(AMPERSAND).collect();
        let mut parameters: Self = Parameters(IndexMap::new());

        for str_param in str_parameters {
            let trimmed_param = str_param.trim();
            if trimmed_param.is_empty() {
                continue;
            }

            let mut parts = trimmed_param.splitn(2, EQUAL);
            let (key, value) = match (parts.next(), parts.next()) {
                (Some(k), Some(v)) => (k, v),
                _ => return Err(Error::InvalidParameter(trimmed_param.into())),
            };

            let trimmed_key = key.trim();
            if trimmed_key.is_empty() || Parameters::EXCLUDE.contains(&trimmed_key) {
                continue;
            }

            let Parameter(similarity, values) = parse_parameter(value)?;
            // Only add parameters that have values
            if values.is_empty() {
                continue;
            }

            parameters
                .0
                .insert(trimmed_key.to_string(), Parameter(similarity, values));
        }

        Ok(parameters)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Parameter(Similarity, Vec<String>);

impl Parameter {
    pub fn init(similarity: Similarity, values: Vec<String>) -> Self {
        Self(similarity, values)
    }

    pub fn similarity(&self) -> &Similarity {
        &self.0
    }

    pub fn values(&self) -> &Vec<String> {
        &self.1
    }

    pub fn values_mut(&mut self) -> &mut Vec<String> {
        &mut self.1
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Order(IndexMap<String, SortOrder>);

impl Order {
    pub fn new() -> Self {
        Self(IndexMap::new())
    }

    pub fn inner(&self) -> &IndexMap<String, SortOrder> {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut IndexMap<String, SortOrder> {
        &mut self.0
    }

    pub fn ascending(&mut self, name: String) -> &mut Self {
        self.0.insert(name, SortOrder::Ascending);
        self
    }

    pub fn descending(&mut self, name: String) -> &mut Self {
        self.0.insert(name, SortOrder::Descending);
        self
    }

    pub fn keep(&self, keys: Vec<String>) -> Self {
        let mut result = Self::new();
        for key in keys {
            if let Some(value) = self.0.get(&key) {
                result.0.insert(key, value.clone());
            }
        }
        result
    }

    pub fn remove(&self, keys: Vec<String>) -> Self {
        let mut result = self.clone();
        for key in keys {
            result.0.shift_remove(&key);
        }
        result
    }
}

impl Default for Order {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for Order {
    type Err = Error;

    // EXAMPLE INPUT
    // date_created:desc,name:asc,surname:asc
    fn from_str(s: &str) -> Result<Self> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Ok(Order::new());
        }

        let str_fields: Vec<&str> = trimmed.split(COMMA).collect();
        let mut order: Self = Order(IndexMap::new());

        for str_field in str_fields {
            let trimmed_field = str_field.trim();
            if trimmed_field.is_empty() {
                continue;
            }

            let (name, sort_order) = parse_order_field(trimmed_field)?;
            order.0.insert(name, sort_order);
        }

        Ok(order)
    }
}

// Utility enums (needed by main types)
#[derive(Clone, Debug, PartialEq)]
pub enum Similarity {
    Equals,
    Contains,
    StartsWith,
    EndsWith,

    Between,
    Lesser,
    LesserOrEqual,
    Greater,
    GreaterOrEqual,
}

impl Similarity {
    pub const EQUALS: &str = "equals";
    pub const CONTAINS: &str = "contains";
    pub const STARTS_WITH: &str = "starts-with";
    pub const ENDS_WITH: &str = "ends-with";

    pub const BETWEEN: &str = "between";
    pub const LESSER: &str = "lesser";
    pub const LESSER_OR_EQUAL: &str = "lesser-or-equal";
    pub const GREATER: &str = "greater";
    pub const GREATER_OR_EQUAL: &str = "greater-or-equal";
}

impl Default for Similarity {
    fn default() -> Self {
        Self::Equals
    }
}

impl FromStr for Similarity {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            Similarity::EQUALS => Ok(Similarity::Equals),
            Similarity::CONTAINS => Ok(Similarity::Contains),
            Similarity::STARTS_WITH => Ok(Similarity::StartsWith),
            Similarity::ENDS_WITH => Ok(Similarity::EndsWith),

            Similarity::BETWEEN => Ok(Similarity::Between),
            Similarity::LESSER => Ok(Similarity::Lesser),
            Similarity::LESSER_OR_EQUAL => Ok(Similarity::LesserOrEqual),
            Similarity::GREATER => Ok(Similarity::Greater),
            Similarity::GREATER_OR_EQUAL => Ok(Similarity::GreaterOrEqual),

            val => Err(Error::InvalidSimilarity(val.into())),
        }
    }
}

impl ToString for Similarity {
    fn to_string(&self) -> String {
        match self {
            Self::Equals => Self::EQUALS.to_string(),
            Self::Contains => Self::CONTAINS.to_string(),
            Self::StartsWith => Self::STARTS_WITH.to_string(),
            Self::EndsWith => Self::ENDS_WITH.to_string(),

            Self::Between => Self::BETWEEN.to_string(),
            Self::Lesser => Self::LESSER.to_string(),
            Self::LesserOrEqual => Self::LESSER_OR_EQUAL.to_string(),
            Self::Greater => Self::GREATER.to_string(),
            Self::GreaterOrEqual => Self::GREATER_OR_EQUAL.to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

impl SortOrder {
    pub const ASCENDING: &str = "asc";
    pub const DESCENDING: &str = "desc";
}

impl Default for SortOrder {
    fn default() -> Self {
        Self::Ascending
    }
}

impl FromStr for SortOrder {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            SortOrder::ASCENDING => Ok(SortOrder::Ascending),
            SortOrder::DESCENDING => Ok(SortOrder::Descending),
            val => Err(Error::InvalidSortOrder(val.into())),
        }
    }
}

impl ToString for SortOrder {
    fn to_string(&self) -> String {
        match self {
            Self::Ascending => SortOrder::ASCENDING.to_string(),
            Self::Descending => SortOrder::DESCENDING.to_string(),
        }
    }
}

// Utility types
#[cfg(feature = "sql")]
#[derive(Clone, Debug, PartialEq)]
pub enum SqlValue {
    /// The value is a `NULL` value.
    Null,
    /// The value is a signed integer.
    Integer(i64),
    /// The value is a floating point number.
    Real(f64),
    /// The value is a text string.
    Text(String),
    /// The value is a blob of data
    Blob(Vec<u8>),
}

// Utility functions
/// Parse a parameter string into similarity and values
///
/// # Examples
/// - "contains:damian" -> (Similarity::Contains, vec!["damian"])
/// - "equals:black,steel,wood" -> (Similarity::Equals, vec!["black", "steel", "wood"])
/// - "between:20,30" -> (Similarity::Between, vec!["20", "30"])
pub(crate) fn parse_parameter(s: &str) -> Result<Parameter> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Err(Error::InvalidParameter(s.into()));
    }

    let parts: Vec<&str> = trimmed.split(COLON).collect();
    if parts.len() != 2 {
        return Err(Error::InvalidParameter(s.into()));
    }

    let similarity_str = parts[0].trim();
    let values_str = parts[1].trim();

    if similarity_str.is_empty() {
        return Err(Error::InvalidParameter(s.into()));
    }

    let values: Vec<String> = if values_str.is_empty() {
        vec![]
    } else {
        values_str
            .split(COMMA)
            .map(|v| url_decode(v.trim()))
            .filter(|v| !v.is_empty())
            .collect()
    };

    let similarity = Similarity::from_str(similarity_str)?;
    Ok(Parameter(similarity, values))
}

/// Parse an order field string into name and order
///
/// # Examples
/// - "name:asc" -> ("name", SortOrder::Ascending)
/// - "date_created:desc" -> ("date_created", SortOrder::Descending)
pub(crate) fn parse_order_field(s: &str) -> Result<(String, SortOrder)> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Err(Error::InvalidSortField(s.into()));
    }

    let parts: Vec<&str> = trimmed.split(COLON).collect();
    if parts.len() != 2 {
        return Err(Error::InvalidSortField(s.into()));
    }

    let name = url_decode(parts[0].trim());
    let order_str = parts[1].trim();

    if name.is_empty() || order_str.is_empty() {
        return Err(Error::InvalidSortField(s.into()));
    }

    let order = SortOrder::from_str(order_str)?;
    Ok((name, order))
}

pub(crate) const QUESTION: char = '?';
pub(crate) const AMPERSAND: char = '&';
pub(crate) const EQUAL: char = '=';
pub(crate) const COLON: char = ':';
pub(crate) const COMMA: char = ',';
pub(crate) const PERCENT: char = '%';

/// URL decode a string, handling percent-encoded characters
pub(crate) fn url_decode(input: &str) -> String {
    // Only decode if the string contains percent-encoded characters
    if input.contains(PERCENT) {
        // Use form_urlencoded to decode individual values by treating it as a query parameter
        let query_str = format!("key={}", input);
        form_urlencoded::parse(query_str.as_bytes())
            .next()
            .map(|(_, v)| v.to_string())
            .unwrap_or_else(|| input.to_string())
    } else {
        input.to_string()
    }
}

/// URL encode a string, converting special characters to percent-encoded format
pub(crate) fn url_encode(input: &str) -> String {
    form_urlencoded::byte_serialize(input.as_bytes()).collect()
}

#[cfg(test)]
mod parse_tests;
