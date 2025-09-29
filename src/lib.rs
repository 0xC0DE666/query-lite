pub mod error;

use error::{Error, Result};
use indexmap::IndexMap;
use std::str::FromStr;
use url::form_urlencoded;

pub const QUESTION: char = '?';
pub const AMPERSAND: char = '&';
pub const EQUAL: char = '=';
pub const COLON: char = ':';
pub const COMMA: char = ',';
pub const PERCENT: char = '%';

/// URL decode a string, handling percent-encoded characters
pub fn url_decode(input: &str) -> String {
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
pub fn url_encode(input: &str) -> String {
    form_urlencoded::byte_serialize(input.as_bytes()).collect()
}

/// Parse a parameter string into similarity and values
///
/// # Examples
/// - "contains:damian" -> (Similarity::Contains, vec!["damian"])
/// - "equals:black,steel,wood" -> (Similarity::Equals, vec!["black", "steel", "wood"])
/// - "between:20,30" -> (Similarity::Between, vec!["20", "30"])
pub fn parse_parameter(s: &str) -> Result<(Similarity, Vec<String>)> {
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
    Ok((similarity, values))
}

/// Parse a sort field string into name and order
///
/// # Examples
/// - "name:asc" -> ("name", SortOrder::Ascending)
/// - "date_created:desc" -> ("date_created", SortOrder::Descending)
pub fn parse_sort_field(s: &str) -> Result<(String, SortOrder)> {
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

#[derive(Clone, Debug, PartialEq)]
pub struct SortFields(pub IndexMap<String, SortOrder>);

impl SortFields {
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

impl Default for SortFields {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for SortFields {
    type Err = Error;

    // EXAMPLE INPUT
    // date_created:desc,name:asc,surname:asc
    fn from_str(s: &str) -> Result<Self> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Ok(SortFields::new());
        }

        let str_fields: Vec<&str> = trimmed.split(COMMA).collect();
        let mut sort_fields: Self = SortFields(IndexMap::new());

        for str_field in str_fields {
            let trimmed_field = str_field.trim();
            if trimmed_field.is_empty() {
                continue;
            }

            let (name, order) = parse_sort_field(trimmed_field)?;
            sort_fields.0.insert(name, order);
        }

        Ok(sort_fields)
    }
}

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

pub type Parameter = (Similarity, Vec<String>);
pub trait ParameterGet {
    fn similarity(&self) -> &Similarity;
    fn values(&self) -> &Vec<String>;
}

impl ParameterGet for Parameter {
    fn similarity(&self) -> &Similarity {
        &self.0
    }

    fn values(&self) -> &Vec<String> {
        &self.1
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Parameters(pub IndexMap<String, Parameter>);

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
        self.0.insert(key, (Similarity::Equals, values));
        self
    }

    pub fn contains(&mut self, key: String, values: Vec<String>) -> &mut Self {
        self.0.insert(key, (Similarity::Contains, values));
        self
    }

    pub fn starts_with(&mut self, key: String, values: Vec<String>) -> &mut Self {
        self.0.insert(key, (Similarity::StartsWith, values));
        self
    }

    pub fn ends_with(&mut self, key: String, values: Vec<String>) -> &mut Self {
        self.0.insert(key, (Similarity::EndsWith, values));
        self
    }

    pub fn between(&mut self, key: String, values: Vec<String>) -> &mut Self {
        self.0.insert(key, (Similarity::Between, values));
        self
    }

    pub fn lesser(&mut self, key: String, values: Vec<String>) -> &mut Self {
        self.0.insert(key, (Similarity::Lesser, values));
        self
    }

    pub fn lesser_or_equal(&mut self, key: String, values: Vec<String>) -> &mut Self {
        self.0.insert(key, (Similarity::LesserOrEqual, values));
        self
    }

    pub fn greater(&mut self, key: String, values: Vec<String>) -> &mut Self {
        self.0.insert(key, (Similarity::Greater, values));
        self
    }

    pub fn greater_or_equal(&mut self, key: String, values: Vec<String>) -> &mut Self {
        self.0.insert(key, (Similarity::GreaterOrEqual, values));
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

            let (similarity, values) = parse_parameter(value)?;
            // Only add parameters that have values
            if values.is_empty() {
                continue;
            }

            parameters
                .0
                .insert(trimmed_key.to_string(), (similarity, values));
        }

        Ok(parameters)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum SQLValue {
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

#[derive(Clone, Debug, PartialEq)]
pub struct Query {
    pub parameters: Parameters,
    pub sort_fields: SortFields,
    pub limit: usize,
    pub offset: usize,
}

impl Query {
    pub fn new() -> Self {
        Self {
            parameters: Parameters::new(),
            sort_fields: SortFields::new(),
            limit: Parameters::DEFAULT_LIMIT,
            offset: Parameters::DEFAULT_OFFSET,
        }
    }

    pub fn init(
        parameters: Parameters,
        sort_fields: SortFields,
        limit: usize,
        offset: usize,
    ) -> Self {
        Self {
            parameters,
            sort_fields,
            limit,
            offset,
        }
    }

    pub fn to_http(&self) -> String {
        let mut params = self
            .parameters
            .0
            .iter()
            .filter(|(_, (_, values))| values.len() > 0)
            .map(|(key, (similarity, values))| {
                let similarity_str = similarity.to_string();
                let values_str = values
                    .iter()
                    .map(|v| url_encode(v))
                    .collect::<Vec<String>>()
                    .join(&format!("{COMMA}"));
                format!("{key}{EQUAL}{similarity_str}{COLON}{values_str}",)
            })
            .collect::<Vec<String>>()
            .join("&");

        let order = self
            .sort_fields
            .0
            .iter()
            .filter(|(name, _)| name.len() > 0)
            .map(|(name, order)| format!("{name}{COLON}{}", order.to_string()))
            .collect::<Vec<String>>()
            .join(&format!("{COMMA}"));

        if params.len() > 0 {
            params.push_str(&format!("{AMPERSAND}"));
        }

        if order.len() > 0 {
            params.push_str(&format!("{}{EQUAL}{}", Parameters::ORDER, order));
            params.push_str(&format!("{AMPERSAND}"));
        }

        format!(
            "{params}{}{EQUAL}{}{AMPERSAND}{}{EQUAL}{}",
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

                if trimmed_key.is_empty() {
                    continue;
                }

                match trimmed_key {
                    Parameters::ORDER => {
                        if trimmed_value.is_empty() {
                            continue;
                        }

                        // Check if the value looks like a sort field format (contains colon)
                        if !trimmed_value.contains(COLON) {
                            // Fail on clearly invalid formats (like "invalid")
                            return Err(Error::InvalidSortField(trimmed_value.into()));
                        }

                        if let Ok(sort_fields) = SortFields::from_str(trimmed_value) {
                            query.sort_fields = sort_fields;
                        }
                        // Skip malformed sort fields (like ":desc")
                    }
                    Parameters::LIMIT => {
                        if trimmed_value.is_empty() {
                            continue;
                        }

                        query.limit = trimmed_value.parse().unwrap_or(Parameters::DEFAULT_LIMIT);
                    }
                    Parameters::OFFSET => {
                        if trimmed_value.is_empty() {
                            continue;
                        }

                        query.offset = trimmed_value.parse().unwrap_or(Parameters::DEFAULT_OFFSET);
                    }
                    _k => {
                        if trimmed_value.is_empty() {
                            continue;
                        }

                        // Check if this is a similarity-based parameter (contains colon)
                        if trimmed_value.contains(COLON) {
                            // Parse as similarity-based parameter
                            let (similarity, values) = parse_parameter(trimmed_value)?;
                            // Only add parameters that have values
                            if values.is_empty() {
                                continue;
                            }
                            // Replace any existing parameter (similarity-based takes precedence)
                            query
                                .parameters
                                .0
                                .insert(trimmed_key.to_string(), (similarity, values));
                        } else {
                            // Handle as normal query parameter (default to equals similarity)
                            let decoded_value = url_decode(trimmed_value);

                            // Check if parameter already exists and is not similarity-based
                            if let Some((existing_similarity, existing_values)) =
                                query.parameters.0.get_mut(&trimmed_key.to_string())
                            {
                                // Only append if the existing parameter is also equals similarity
                                if *existing_similarity == Similarity::Equals {
                                    existing_values.push(decoded_value);
                                }
                                // If existing parameter is similarity-based, ignore this normal parameter
                            } else {
                                // Create new parameter with equals similarity
                                query.parameters.0.insert(
                                    trimmed_key.to_string(),
                                    (Similarity::Equals, vec![decoded_value]),
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
        let where_clause = self.build_where_clause();
        if !where_clause.is_empty() {
            sql_parts.push(format!("WHERE {}", where_clause));
        }

        // Build ORDER BY clause from sort fields
        let order_clause = self.build_order_clause();
        if !order_clause.is_empty() {
            sql_parts.push(format!("ORDER BY {}", order_clause));
        }

        // Add LIMIT and OFFSET
        sql_parts.push(format!("LIMIT ? OFFSET ?"));

        sql_parts.join(" ")
    }

    #[cfg(feature = "sql")]
    fn build_where_clause(&self) -> String {
        let mut conditions = Vec::new();

        for (key, (similarity, values)) in &self.parameters.0 {
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

        conditions.join(" AND ")
    }

    #[cfg(feature = "sql")]
    fn build_order_clause(&self) -> String {
        let mut order_parts = Vec::new();

        for (name, order) in &self.sort_fields.0 {
            if !name.is_empty() {
                let direction = match order {
                    SortOrder::Ascending => "ASC",
                    SortOrder::Descending => "DESC",
                };
                order_parts.push(format!("{} {}", name, direction));
            }
        }

        order_parts.join(", ")
    }

    #[cfg(feature = "sql")]
    pub fn to_values(&self) -> Result<Vec<SQLValue>> {
        let mut sql_values = Vec::new();

        for k in self.parameters.inner().keys() {
            let (param_similarity, param_values) =
                self.parameters.inner().get(k).ok_or_else(|| {
                    Error::InvalidSQLValue(format!("Parameter '{}' not found", k))
                })?;

            for cur_val in param_values {
                if cur_val == "null" {
                    sql_values.push(SQLValue::Null);
                    continue;
                }

                let sql_value = match *param_similarity {
                    Similarity::Contains => SQLValue::Text(format!("%{}%", cur_val)),
                    Similarity::StartsWith => SQLValue::Text(format!("{}%", cur_val)),
                    Similarity::EndsWith => SQLValue::Text(format!("%{}", cur_val)),
                    _ => {
                        // Try to parse as integer first, then float, then text
                        if let Ok(i) = cur_val.parse::<i64>() {
                            SQLValue::Integer(i)
                        } else if let Ok(f) = cur_val.parse::<f64>() {
                            SQLValue::Real(f)
                        } else {
                            SQLValue::Text(cur_val.clone())
                        }
                    }
                };

                sql_values.push(sql_value);
            }
        }

        // Add limit and offset as the last two parameters
        sql_values.push(SQLValue::Integer(self.limit as i64));
        sql_values.push(SQLValue::Integer(self.offset as i64));

        Ok(sql_values)
    }

    #[cfg(feature = "sql")]
    /// Get SQL values for parameters only (without limit and offset)
    pub fn to_parameter_values(&self) -> Result<Vec<SQLValue>> {
        let mut sql_values = Vec::new();

        for k in self.parameters.inner().keys() {
            let (param_similarity, param_values) =
                self.parameters.inner().get(k).ok_or_else(|| {
                    Error::InvalidSQLValue(format!("Parameter '{}' not found", k))
                })?;

            for cur_val in param_values {
                if cur_val == "null" {
                    sql_values.push(SQLValue::Null);
                    continue;
                }

                let sql_value = match *param_similarity {
                    Similarity::Contains => SQLValue::Text(format!("%{}%", cur_val)),
                    Similarity::StartsWith => SQLValue::Text(format!("{}%", cur_val)),
                    Similarity::EndsWith => SQLValue::Text(format!("%{}", cur_val)),
                    _ => {
                        // Try to parse as integer first, then float, then text
                        if let Ok(i) = cur_val.parse::<i64>() {
                            SQLValue::Integer(i)
                        } else if let Ok(f) = cur_val.parse::<f64>() {
                            SQLValue::Real(f)
                        } else {
                            SQLValue::Text(cur_val.clone())
                        }
                    }
                };

                sql_values.push(sql_value);
            }
        }

        Ok(sql_values)
    }

    #[cfg(feature = "sql")]
    /// Get SQL values for pagination (limit and offset only)
    pub fn to_pagination_values(&self) -> Vec<SQLValue> {
        vec![
            SQLValue::Integer(self.limit as i64),
            SQLValue::Integer(self.offset as i64),
        ]
    }

    #[cfg(feature = "sql")]
    /// Get the total number of SQL parameter values (parameters + pagination)
    pub fn total_parameters(&self) -> usize {
        let parameter_count: usize = self
            .parameters
            .inner()
            .values()
            .map(|(_, values)| values.len())
            .sum();

        parameter_count + 2 // +2 for limit and offset
    }
}
