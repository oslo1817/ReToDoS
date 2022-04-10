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
    /// Add a new ToDo item to the list.
    Add {
        /// The title of the ToDo item.
        title: String,

        /// The due date of the ToDo item.
        due_date: String,
    },

    /// Delete a ToDo item from the list (i.e. complete the task).
    Delete {
        /// The number as shown by `retodos list` of the ToDo item to delete.
        ordinal: usize,
    },

    /// List all ToDo items.
    List,

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
    Info {
        /// The section of the Redis info to show.
        #[clap(default_value = "server")]
        section: String,
    },
}

/// Parses the command line arguments and returns the result.
pub fn parse() -> Cli {
    use clap::Parser;
    return Cli::parse();
}
