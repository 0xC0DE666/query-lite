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

/// URL decode a string, handling percent-encoded characters
pub fn url_decode(input: &str) -> String {
    // Only decode if the string contains percent-encoded characters
    if input.contains('%') {
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
pub struct SortField {
    pub name: String,
    pub order: SortOrder,
}

impl SortField {
    pub fn init(name: String, order: SortOrder) -> Self {
        Self { name, order }
    }
}

impl FromStr for SortField {
    type Err = Error;

    // EXAMPLE INPUT
    // date_created:desc
    // name:asc
    // surname:asc
    fn from_str(s: &str) -> Result<Self> {
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

        Ok(SortField::init(
            name,
            SortOrder::from_str(order_str)?,
        ))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SortFields(pub Vec<SortField>);

impl SortFields {
    pub fn new() -> Self {
        Self(Vec::new())
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
        let mut sort_fields: Self = SortFields(vec![]);

        for str_field in str_fields {
            let trimmed_field = str_field.trim();
            if !trimmed_field.is_empty() {
                sort_fields.0.push(SortField::from_str(trimmed_field)?);
            }
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
}

impl Similarity {
    pub const EQUALS: &str = "equals";
    pub const CONTAINS: &str = "contains";
    pub const STARTS_WITH: &str = "starts-with";
    pub const ENDS_WITH: &str = "ends-with";
}

impl Default for Similarity {
    fn default() -> Self {
        Self::Contains
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
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Parameter {
    pub similarity: Similarity,
    pub values: Vec<String>,
}

impl Parameter {
    pub fn new() -> Self {
        Self {
            similarity: Similarity::default(),
            values: vec![],
        }
    }

    pub fn init(similarity: Similarity, values: Vec<String>) -> Self {
        Self { similarity, values }
    }
}

impl FromStr for Parameter {
    type Err = Error;

    // EXAMPLE INPUT
    // name=contains:damian
    // name=equals:black,steel,wood
    // name=starts-with:black,steel,wood
    // name=ends-with:black,steel,wood
    fn from_str(s: &str) -> Result<Self> {
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

        Ok(Parameter::init(
            Similarity::from_str(similarity_str)?,
            values,
        ))
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

    pub const MAX_LIMIT: usize = 100;

    pub fn new() -> Self {
        Self(IndexMap::new())
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
            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                let trimmed_key = key.trim();
                if trimmed_key.is_empty() || Parameters::EXCLUDE.contains(&trimmed_key) {
                    continue;
                }
                let param = Parameter::from_str(value)?;
                // Only add parameters that have values
                if !param.values.is_empty() {
                    parameters.0.insert(trimmed_key.to_string(), param);
                }
            } else {
                return Err(Error::InvalidParameter(trimmed_param.into()));
            }
        }

        Ok(parameters)
    }
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
            .filter(|(_, param)| param.values.len() > 0)
            .map(|(key, param)| {
                let similarity = param.similarity.to_string();
                let values = param.values
                    .iter()
                    .map(|v| url_encode(v))
                    .collect::<Vec<String>>()
                    .join(&format!("{COMMA}"));
                format!("{key}{EQUAL}{similarity}{COLON}{values}",)
            })
            .collect::<Vec<String>>()
            .join("&");

        let order = self
            .sort_fields
            .0
            .iter()
            .filter(|field| field.name.len() > 0)
            .map(|field| {
                let name = field.name.clone();
                let order = field.order.to_string();
                format!("{name}{COLON}{order}")
            })
            .collect::<Vec<String>>()
            .join(&format!("{COMMA}"));

        if params.len() > 0 {
            params.push_str(&format!("{AMPERSAND}"));
        }

        if order.len() > 0 {
            params.push_str(&order);
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
                        if !trimmed_value.is_empty() {
                            // Check if the value looks like a sort field format (contains colon)
                            if trimmed_value.contains(':') {
                                if let Ok(sort_fields) = SortFields::from_str(trimmed_value) {
                                    query.sort_fields = sort_fields;
                                }
                                // Skip malformed sort fields (like ":desc")
                            } else {
                                // Fail on clearly invalid formats (like "invalid")
                                return Err(Error::InvalidSortField(trimmed_value.into()));
                            }
                        }
                    }
                    Parameters::LIMIT => {
                        if !trimmed_value.is_empty() {
                            let limit: usize =
                                trimmed_value.parse().unwrap_or(Parameters::DEFAULT_LIMIT);
                            query.limit = limit.min(Parameters::MAX_LIMIT);
                        }
                    }
                    Parameters::OFFSET => {
                        if !trimmed_value.is_empty() {
                            query.offset =
                                trimmed_value.parse().unwrap_or(Parameters::DEFAULT_OFFSET);
                        }
                    }
                    _k => {
                        if !trimmed_value.is_empty() {
                            let param = Parameter::from_str(trimmed_value)?;
                            // Only add parameters that have values
                            if !param.values.is_empty() {
                                query.parameters.0.insert(trimmed_key.to_string(), param);
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

    pub fn keep(&self, keys: Vec<String>) -> Self {
        let mut clone = self.clone();
        let keys_to_remove: Vec<String> = self
            .parameters
            .0
            .keys()
            .filter(|k| !keys.contains(k))
            .map(|k| k.clone())
            .collect();

        for k in keys_to_remove {
            clone.parameters.0.shift_remove(&k);
        }

        clone
    }

    pub fn remove(&self, keys: Vec<String>) -> Self {
        let mut clone = self.clone();
        let keys_to_remove: Vec<String> = self
            .parameters
            .0
            .keys()
            .filter(|k| keys.contains(k))
            .map(|k| k.clone())
            .collect();

        for k in keys_to_remove {
            clone.parameters.0.shift_remove(&k);
        }

        clone
    }
}
