mod cli;
mod data;

use chrono::Utc;
use chrono_english::{parse_date_string, DateError, Dialect};
use cli::{Command, RedisCommand};
use data::model::ToDoItem;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::parse();
    let mut manager = data::Manager::new();

    match &cli.command {
        Command::Add { title, due_date } => {
            manager.add_item(&ToDoItem::from(title, due_date)?)?;
        }

        Command::Redis { command } => match &command {
            RedisCommand::Info { section } => {
                println!("{}", manager.get_redis_info(section)?);
            }
        },
    }

    Ok(())
}

impl ToDoItem {
    /// Creates a new [ToDoItem] from a title and a due date.
    fn from(title: &String, due_date: &String) -> Result<Self, DateError> {
        let title = title.to_string();
        let due_date = parse_date_string(due_date, Utc::now(), Dialect::Us)?;

        Ok(Self { title, due_date })
    }
}
