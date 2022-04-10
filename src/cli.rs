use clap;

#[derive(clap::Parser)]
#[clap(about, version)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

/// The allowed subcommands for the CLI.
#[derive(clap::Subcommand)]
pub enum Command {}

/// Parses the command line arguments and returns the result.
pub fn parse() -> Cli {
    use clap::Parser;
    return Cli::parse();
}
