pub mod model;
use crate::args::InputPath;
use crate::fs;
use anyhow::Result;
use chrono::{DateTime, Local};
use reqwest::blocking::Client;
use serde::Serialize;

#[derive(Serialize)]
struct SeriesParams {
    #[serde(rename = "match[]")]
    match_param: String,
    start: String,
    end: String,
}

pub fn read(
    input: InputPath,
    start: DateTime<Local>,
    end: DateTime<Local>,
    api_path: &str,
) -> Result<Vec<u8>> {
    match input {
        InputPath::File(path) => fs::read_file(path),
        InputPath::Url(url) => {
            let client = Client::new();

            let params = SeriesParams {
                start: start.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
                end: end.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
                match_param: "{__name__=~\".+?\"}".to_string(),
            };

            let url = format!("{}{}", url.as_str().trim_end_matches('/'), api_path);
            let response = client.get(url).query(&params).send()?;

            Ok(response.bytes()?.to_vec())
        }
    }
}
