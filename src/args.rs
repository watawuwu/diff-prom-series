use clap::builder::{styling, Styles};
use clap::Parser;
use std::path::PathBuf;

fn help_styles() -> Styles {
    styling::Styles::styled()
        .header(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .usage(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .literal(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
        .placeholder(styling::AnsiColor::Cyan.on_default())
}

#[derive(Parser, Debug)]
#[command(author, version, about, next_line_help = true, long_about = None, styles(help_styles()))]
pub struct Args {
    #[arg(name = "FILE1")]
    pub file1: PathBuf,

    #[arg(name = "FILE2")]
    pub file2: PathBuf,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Args::command().debug_assert()
}
