use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{LazyLock, RwLock};

pub static DISABLE_ADJUST_BUCKET_LABEL: LazyLock<RwLock<bool>> =
    LazyLock::new(|| RwLock::new(false));

#[derive(Debug, Serialize, Deserialize)]
pub struct PrometheusSeriesResponse {
    status: String,
    pub data: BTreeSet<Series>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct Series {
    #[serde(rename = "__name__")]
    pub name: String,
    #[serde(flatten, deserialize_with = "custom_transform")]
    pub labels: BTreeMap<String, String>,
}

fn custom_transform<'de, D>(deserializer: D) -> Result<BTreeMap<String, String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: BTreeMap<String, String> = Deserialize::deserialize(deserializer)?;

    if *DISABLE_ADJUST_BUCKET_LABEL.read().unwrap() {
        return Ok(value);
    }

    let value = value
        .into_iter()
        .map(|(k, v)| {
            if (&k == "le" && &v != "+Inf") || &k == "quantile" {
                let v = v.parse::<f64>().map_err(de::Error::custom)?;
                let v = format!("{:.1}", v);

                return Ok((k, v));
            }
            Ok((k, v))
        })
        .collect::<Result<BTreeMap<_, _>, D::Error>>()?;

    Ok(value)
}

impl std::hash::Hash for Series {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.labels.hash(state);
    }
}
