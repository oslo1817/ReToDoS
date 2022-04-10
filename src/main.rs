mod cli;
mod data;

use cli::{Command, RedisCommand};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::parse();
    let mut manager = data::Manager::new();

    match &cli.command {
        Command::Redis { command } => match &command {
            RedisCommand::Info => {
                println!("{}", manager.get_redis_server_info()?);
            }
        },
    }

    Ok(())
}
