use colored_json::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fmt::{Display, Formatter},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct PrometheusSeriesResponse {
    status: String,
    pub data: BTreeSet<Series>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct Series {
    #[serde(rename = "__name__")]
    pub name: String,
    #[serde(flatten)]
    pub labels: BTreeMap<String, String>,
}

impl std::hash::Hash for Series {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.labels.hash(state);
    }
}

impl Display for Series {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let value = serde_json::to_value(self).map_err(|_| std::fmt::Error)?;
        let buf = colored_json::to_colored_json_auto(&value).map_err(|_| std::fmt::Error)?;
        write!(f, "{}", buf)
    }
}
