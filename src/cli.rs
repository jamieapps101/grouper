use clap::{Parser, ValueEnum};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Mode {
    /// Attempt to split the line assuming the key is "quoted" and the value is not. Space immediately after the quoted key will be respected
    Quoted,
    /// Attempt to split the line assuming the key is all the text prior to the first whitespace encountered, and the value is everything else. The whitespace will be removed.
    WhiteSpace,
}

/// Utility to group lines from stdin based on "keys" and "values"
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Number of times to greet
    #[arg(value_enum, default_value_t = Mode::WhiteSpace)]
    pub mode: Mode,
}
