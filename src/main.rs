#![allow(dead_code, unused_imports, unused_variables, while_true)]
mod args;
mod fs;
mod prometheus;

use crate::prometheus::model::PrometheusSeriesResponse;
use anyhow::{self, Result};
use args::Args;
use clap::Parser;
use colored_json::prelude::*;
use prometheus::model::Series;
use std::collections::BTreeSet;
use std::{collections::HashMap, path::PathBuf};

fn main() -> Result<()> {
    pretty_env_logger::init();
    let args = Args::parse();

    let buf1 = fs::read_file(args.file1)?;
    let buf2 = fs::read_file(args.file2)?;

    let (diff1, diff2) = run(buf1, buf2)?;
    show(diff1, diff2);

    Ok(())
}

fn run(buf1: Vec<u8>, buf2: Vec<u8>) -> Result<(BTreeSet<Series>, BTreeSet<Series>)> {
    let res1: PrometheusSeriesResponse = serde_json::from_slice(&buf1)?;
    let res2: PrometheusSeriesResponse = serde_json::from_slice(&buf2)?;

    let series1 = res1.data;
    let series2 = res2.data;

    let diff1 = series1
        .difference(&series2)
        .cloned()
        .collect::<BTreeSet<_>>();
    let diff2 = series2
        .difference(&series1)
        .cloned()
        .collect::<BTreeSet<_>>();

    Ok((diff1, diff2))
}

fn show(diff1: BTreeSet<Series>, diff2: BTreeSet<Series>) {
    println!("Only file1 series count:{}", diff1.len());
    for series in diff1 {
        println!("{}", series);
    }

    println!("Only file2 series count:{}", diff2.len());
    for series in diff2 {
        println!("{}", series);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Context, Result};

    #[test]
    fn test() -> Result<()> {
        let buf1 = r#"{
  "status": "success",
  "data": [
    {
      "__name__": "prometheus_http_request_duration_seconds_bucket",
      "handler": "/api/v1/series",
      "instance": "localhost:9090",
      "job": "prometheus",
      "le": "0.1"
    },
    {
      "__name__": "prometheus_http_request_duration_seconds_bucket",
      "handler": "/api/v1/series",
      "instance": "localhost:9090",
      "job": "prometheus",
      "le": "0.2"
    },
    {
      "__name__": "prometheus_http_request_duration_seconds_bucket",
      "handler": "/api/v1/series",
      "instance": "localhost:9090",
      "job": "prometheus",
      "le": "0.3"
    }
  ]
}
            "#;

        let buf2 = r#"{
  "status": "success",
  "data": [
    {
      "__name__": "prometheus_http_request_duration_seconds_bucket",
      "handler": "/api/v1/series",
      "instance": "localhost:9090",
      "job": "prometheus",
      "le": "0.2"
    },
    {
      "__name__": "prometheus_http_request_duration_seconds_bucket",
      "handler": "/api/v1/series",
      "instance": "localhost:9090",
      "job": "prometheus",
      "le": "0.3"
    },
    {
      "__name__": "prometheus_http_request_duration_seconds_bucket",
      "handler": "/api/v1/series",
      "instance": "localhost:9090",
      "job": "prometheus",
      "le": "0.4"
    },
    {
      "__name__": "prometheus_http_request_duration_seconds_bucket",
      "handler": "/api/v1/series",
      "instance": "localhost:9090",
      "job": "prometheus",
      "le": "0.5"
    }
  ]
}"#;

        let buf1 = buf1.as_bytes().to_vec();
        let buf2 = buf2.as_bytes().to_vec();

        let (d1, d2) = run(buf1, buf2)?;
        assert!(d1.len() == 1);
        assert!(d2.len() == 2);

        let mut it = d1.iter();
        let s = it.next().unwrap();
        let le = s.labels.get("le").unwrap();
        assert_eq!(le, "0.1");

        let mut it = d2.iter();
        let s = it.next().unwrap();
        let le = s.labels.get("le").unwrap();
        assert_eq!(le, "0.4");

        let s = it.next().unwrap();
        let le = s.labels.get("le").unwrap();
        assert_eq!(le, "0.5");

        Ok(())
    }
}
