use std::collections::BTreeSet;

use crate::prometheus::model::{PrometheusSeriesResponse, Series};
use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DiffResult {
    pub only_from: BTreeSet<Series>,
    pub only_to: BTreeSet<Series>,
}

pub fn diff(from_buf: Vec<u8>, to_buf: Vec<u8>) -> Result<DiffResult> {
    let from_res: PrometheusSeriesResponse = serde_json::from_slice(&from_buf)?;
    let to_res: PrometheusSeriesResponse = serde_json::from_slice(&to_buf)?;

    let from_series = from_res.data;
    let to_series = to_res.data;

    let only_from = from_series
        .difference(&to_series)
        .cloned()
        .collect::<BTreeSet<_>>();
    let only_to = to_series
        .difference(&from_series)
        .cloned()
        .collect::<BTreeSet<_>>();

    Ok(DiffResult { only_from, only_to })
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test() -> Result<()> {
        let from = r#"{
  "status": "success",
  "data": [
    {
      "__name__": "prometheus_http_request_duration_seconds_bucket",
      "handler": "/api/v1/series",
      "instance": "localhost:9090",
      "job": "prometheus",
      "le": "1"
    },
    {
      "__name__": "prometheus_http_request_duration_seconds_bucket",
      "handler": "/api/v1/series",
      "instance": "localhost:9090",
      "job": "prometheus",
      "le": "2"
    },
    {
      "__name__": "prometheus_http_request_duration_seconds_bucket",
      "handler": "/api/v1/series",
      "instance": "localhost:9090",
      "job": "prometheus",
      "le": "3"
    }
  ]
}
            "#;

        let to = r#"{
  "status": "success",
  "data": [
    {
      "__name__": "prometheus_http_request_duration_seconds_bucket",
      "handler": "/api/v1/series",
      "instance": "localhost:9090",
      "job": "prometheus",
      "le": "2.0"
    },
    {
      "__name__": "prometheus_http_request_duration_seconds_bucket",
      "handler": "/api/v1/series",
      "instance": "localhost:9090",
      "job": "prometheus",
      "le": "3.0"
    },
    {
      "__name__": "prometheus_http_request_duration_seconds_bucket",
      "handler": "/api/v1/series",
      "instance": "localhost:9090",
      "job": "prometheus",
      "le": "4.0"
    },
    {
      "__name__": "prometheus_http_request_duration_seconds_bucket",
      "handler": "/api/v1/series",
      "instance": "localhost:9090",
      "job": "prometheus",
      "le": "5.0"
    }
  ]
}"#;

        let from_buf = from.as_bytes().to_vec();
        let to_buf = to.as_bytes().to_vec();

        let DiffResult { only_from, only_to } = diff(from_buf, to_buf)?;
        assert!(only_from.len() == 1);
        assert!(only_to.len() == 2);

        let mut it = only_from.iter();
        let s = it.next().unwrap();
        let le = s.labels.get("le").unwrap();
        assert_eq!(le, "1.0");

        let mut it = only_to.iter();
        let s = it.next().unwrap();
        let le = s.labels.get("le").unwrap();
        assert_eq!(le, "4.0");

        let s = it.next().unwrap();
        let le = s.labels.get("le").unwrap();
        assert_eq!(le, "5.0");

        Ok(())
    }
}
