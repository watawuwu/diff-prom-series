use clap::builder::{styling, Styles};
use clap::{Parser, ValueEnum};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use strum::AsRefStr;

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
    /// Output format
    #[arg(short, long, default_value_t = OutputFormat::Text)]
    pub output: OutputFormat,

    #[arg(name = "FROM_FILE")]
    pub from_file: PathBuf,

    #[arg(name = "TO_FILE")]
    pub to_file: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, ValueEnum, AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum OutputFormat {
    Text,
    Json,
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Args::command().debug_assert()
}
