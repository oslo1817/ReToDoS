use clap;

#[derive(clap::Parser)]
#[clap(about, version)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

/// The allowed subcommands for the CLI.
#[derive(clap::Subcommand)]
pub enum Command {
    /// Show and manage the connection to Redis.
    Redis {
        #[clap(subcommand)]
        command: RedisCommand,
    },
}

/// The allowed subcommands for the Redis command.
#[derive(clap::Subcommand)]
pub enum RedisCommand {
    /// Show details about the Redis server.
    Info,
}

/// Parses the command line arguments and returns the result.
pub fn parse() -> Cli {
    use clap::Parser;
    return Cli::parse();
}
