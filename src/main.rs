mod args;
mod diff;
mod fs;
mod prometheus;
mod view;

use crate::diff::diff;
use anyhow::Result;
use args::Args;
use clap::Parser;
use view::output;

fn main() -> Result<()> {
    pretty_env_logger::init();
    let args = Args::parse();

    let from_buf = fs::read_file(args.from_file)?;
    let to_buf = fs::read_file(args.to_file)?;

    let result = diff(from_buf, to_buf)?;
    println!("{}", output(result, args.output)?);

    Ok(())
}
