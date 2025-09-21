// ####################
// QUERY MODEL
// ####################

mod utils;

use indexmap::IndexMap;
use utils::csv;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Order {
    Ascending,
    Descending,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Query {
    pub params: IndexMap<String, Vec<String>>,
    pub limit: usize,
    pub offset: usize,
}

impl Query {
    pub const ORDER: &str = "order";
    pub const LIMIT: &str = "limit";
    pub const OFFSET: &str = "offset";

    pub const DEFAULT_LIMIT: usize = 40;
    pub const DEFAULT_OFFSET: usize = 0;

    pub const MAX_LIMIT: usize = 100;

    pub fn new() -> Self {
        Self {
            params: IndexMap::new(),
            limit: Self::DEFAULT_LIMIT,
            offset: Self::DEFAULT_OFFSET,
        }
    }

    pub fn init(limit: usize, offset: usize) -> Self {
        Self {
            params: IndexMap::new(),
            limit,
            offset,
        }
    }

    pub fn default_http() -> String {
        format!(
            "{}={}&{}={}",
            Self::LIMIT,
            Self::DEFAULT_LIMIT,
            Self::OFFSET,
            Self::DEFAULT_OFFSET,
        )
    }

    pub fn to_http(&self) -> String {
        let mut search = self
            .params
            .iter()
            .filter(|(k, v)| v.len() > 0)
            .map(|(k, vec)| format!("{k}={}", csv::from_vec(vec)))
            .collect::<Vec<String>>()
            .join("&");

        if search.len() > 0 {
            search.push_str("&");
        }

        format!(
            "{search}{}={}&{}={}",
            Self::LIMIT,
            self.limit,
            Self::OFFSET,
            self.offset,
        )
    }

    // order=date_created:desc,name:asc
    // name=damian:equal
    // name=damian:like
    // name=damian,daemon:equal&
    pub fn from_http(search: String) -> Self {
        let mut query = Self::new();
        let search = search.trim_start_matches('?');

        for pair in search.split('&') {
            let mut parts = pair.splitn(2, '=');
            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                match key {
                    Self::LIMIT => {
                        let limit = value.parse().unwrap_or(0);
                        query.limit = limit;
                    }
                    Self::OFFSET => query.offset = value.parse().unwrap_or(0),
                    _ => {
                        let values = value.split(',').map(|s| s.to_string()).collect();
                        query.params.insert(key.to_string(), values);
                    }
                }
            }
        }

        query
    }

    pub fn keep(&self, keys: Vec<String>) -> Self {
        let mut clone = self.clone();
        for k in self.params.keys() {
            if keys.contains(k) == false {
                clone.params.shift_remove(k);
            }
        }

        clone
    }

    pub fn remove(&self, keys: Vec<String>) -> Self {
        let mut clone = self.clone();
        for k in self.params.keys() {
            if keys.contains(k) == true {
                clone.params.shift_remove(k);
            }
        }

        clone
    }
}
