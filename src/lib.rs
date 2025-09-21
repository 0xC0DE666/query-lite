mod error;

use error::{Error, Result};
use indexmap::IndexMap;
use std::str::FromStr;

pub const QUESTION: char = '?';
pub const AMPERSAND: char = '&';
pub const EQUALS: char = '=';
pub const COLON: char = ':';
pub const COMMA: char = ',';

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
        let parts: Vec<&str> = s.split(COLON).collect();
        if parts.len() != 2 {
            return Err(Error::InvalidSortField(s.into()));
        }

        Ok(SortField::init(
            parts[0].into(),
            SortOrder::from_str(parts[1])?,
        ))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SortFields(Vec<SortField>);

impl SortFields {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl FromStr for SortFields {
    type Err = Error;

    // EXAMPLE INPUT
    // date_created:desc,name:asc,surname:asc
    fn from_str(s: &str) -> Result<Self> {
        let str_fields: Vec<&str> = s.split(COMMA).collect();
        let mut sort_fields: Self = SortFields(vec![]);

        for str in str_fields {
            sort_fields.0.push(SortField::from_str(str)?);
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
            val => Err(Error::InvalidSimilaritty(val.into())),
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
    // name=cotains:damian
    // name=equals:black,steel,wood
    // name=starts-with:black,steel,wood
    // name=ends-with:black,steel,wood
    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split(COLON).collect();
        if parts.len() != 2 {
            return Err(Error::InvalidParameter(s.into()));
        }

        Ok(Parameter::init(
            Similarity::from_str(parts[0])?,
            parts[1].split(COMMA).map(String::from).collect(),
        ))
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

    pub const MAX_LIMIT: usize = 100;

    pub fn new() -> Self {
        Self(IndexMap::new())
    }
}

impl FromStr for Parameters {
    type Err = Error;

    // EXAMPLE INPUT
    // name=contains:damian&surname=equals:black,steel,wood&order=date_created:desc&limit=40&offset=0
    fn from_str(s: &str) -> Result<Self> {
        let str_parameters: Vec<&str> = s.split(AMPERSAND).collect();
        let mut parameters: Self = Parameters(IndexMap::new());

        for str in str_parameters {
            if Parameters::EXCLUDE.contains(&str) {
                continue;
            }
            parameters.0.insert(str.into(), Parameter::from_str(str)?);
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

    pub fn init(limit: usize, offset: usize) -> Self {
        Self {
            parameters: Parameters::new(),
            sort_fields: SortFields::new(),
            limit,
            offset,
        }
    }

    pub fn default_http() -> String {
        format!(
            "{}={}&{}={}",
            Parameters::LIMIT,
            Parameters::DEFAULT_LIMIT,
            Parameters::OFFSET,
            Parameters::DEFAULT_OFFSET,
        )
    }

    pub fn to_http(&self) -> String {
        let mut search = self
            .parameters
            .0
            .iter()
            .filter(|(_, param)| param.values.len() > 0)
            .map(|(k, param)| format!("{k}={}", param.values.join(&format!("{COMMA}"))))
            .collect::<Vec<String>>()
            .join("&");

        if search.len() > 0 {
            search.push_str("&");
        }

        format!(
            "{search}{}={}&{}={}",
            Parameters::LIMIT,
            self.limit,
            Parameters::OFFSET,
            self.offset,
        )
    }

    // name=contains:damian&surname=equals:black,steel,wood&order=date_created:desc&limit=40&offset=0
    pub fn from_http(search: String) -> Result<Self> {
        let mut query = Self::new();
        let trimmed_search = search.trim_start_matches(QUESTION);

        for k_v in trimmed_search.split(AMPERSAND) {
            let mut parts = k_v.splitn(2, EQUALS);
            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                match key {
                    Parameters::ORDER => {
                        query.sort_fields = SortFields::from_str(value)?;
                    }
                    Parameters::LIMIT => {
                        query.limit = value.parse().unwrap_or(Parameters::DEFAULT_LIMIT);
                    }
                    Parameters::OFFSET => {
                        query.offset = value.parse().unwrap_or(Parameters::DEFAULT_OFFSET)
                    }
                    k => {
                        query
                            .parameters
                            .0
                            .insert(k.into(), Parameter::from_str(value)?);
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
        for k in self.parameters.0.keys() {
            if keys.contains(k) == false {
                clone.parameters.0.shift_remove(k);
            }
        }

        clone
    }

    pub fn remove(&self, keys: Vec<String>) -> Self {
        let mut clone = self.clone();
        for k in self.parameters.0.keys() {
            if keys.contains(k) == true {
                clone.parameters.0.shift_remove(k);
            }
        }

        clone
    }
}
