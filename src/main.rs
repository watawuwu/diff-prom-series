mod args;
mod diff;
mod fs;
mod prometheus;
mod view;

use crate::diff::diff;
use anyhow::Result;
use args::Args;
use clap::Parser;
use prometheus::model::DISABLE_ADJUST_BUCKET_LABEL;
use view::output;

fn main() -> Result<()> {
    pretty_env_logger::init();
    let args = Args::parse();

    // Workaround
    {
        let mut opt = DISABLE_ADJUST_BUCKET_LABEL.write().unwrap();
        *opt = args.disable_adjust_bucket_label;
    }

    let from_buf = prometheus::read(
        args.from_input,
        args.from_start,
        args.from_end,
        &args.api_path,
    )?;
    let to_buf = prometheus::read(args.to_input, args.to_start, args.to_end, &args.api_path)?;

    let result = diff(from_buf, to_buf)?;
    println!("{}", output(result, args.output)?);

    Ok(())
}
