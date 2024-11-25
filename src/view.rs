use crate::args::OutputFormat;
use crate::diff::DiffResult;
use anyhow::Result;
use colored_json::prelude::*;

pub fn output(result: DiffResult, format: OutputFormat) -> Result<String> {
    let buf = match format {
        OutputFormat::Text => text(result)?,
        OutputFormat::Json => json(result)?,
    };
    Ok(buf)
}

fn text(result: DiffResult) -> Result<String> {
    let from = result.only_from;
    let to = result.only_to;

    let mut outputs = Vec::new();

    outputs.push(format!("Only from_file series count:{}", from.len()));
    let json = serde_json::to_string(&from)?.to_colored_json_auto()?;
    outputs.push(json);

    outputs.push(format!("Only from_file series count:{}", to.len()));
    let json = serde_json::to_string(&to)?.to_colored_json_auto()?;
    outputs.push(json);

    Ok(outputs.join("\n"))
}

fn json(result: DiffResult) -> Result<String> {
    Ok(serde_json::to_string_pretty(&result)?)
}
