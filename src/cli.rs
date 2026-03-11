use clap::Parser;

/// CLI arguments for the password generator.
#[derive(Parser)]
#[command(
    name = "spg",
    version,
    author,
    about = "A fast, simple CLI tool to generate secure passwords"
)]
pub struct Cli {
    /// Length of the generated password (default: 12)
    #[arg(short, long, default_value_t = 12)]
    pub length: usize,

    /// Include special symbols in the password (default: true)
    #[arg(short, long, default_value_t = true)]
    pub symbols: bool,
}

/// Parses CLI arguments using clap.
pub fn parse_args() -> Cli {
    Cli::parse()
}
