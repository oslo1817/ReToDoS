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
        /// Delete all ToDo items. If present, ORDINAL is ignored.
        #[clap(long, short)]
        all: bool,

        /// The number as shown by `retodos list` of the ToDo item to delete.
        #[clap(required_unless_present("all"))]
        ordinal: Option<usize>,
    },

    /// Show all ToDo items in the list.
    List,

    /// Show and manage the connection to Redis.
    Redis {
        #[clap(subcommand)]
        command: RedisCommand,
    },

    /// Update a ToDo item in the list.
    Update {
        /// The number as shown by `retodos list` of the ToDo item to update.
        ordinal: usize,

        /// The new title of the ToDo item.
        #[clap(long, short)]
        title: Option<String>,

        /// The new due date of the ToDo item.
        #[clap(long, short)]
        due_date: Option<String>,
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
