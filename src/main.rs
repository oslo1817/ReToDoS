mod cli;
mod data;

use chrono::{Local, Utc};
use chrono_english::{parse_date_string, DateError, Dialect};
use cli::{Command, RedisCommand};
use data::model::ToDoItem;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::parse();
    let mut manager = data::Manager::new();

    match &cli.command {
        Command::Add { title, due_date } => {
            manager.add_item(&ToDoItem::from(title, due_date)?)?;
            println!("Added: \"{}\"", title);
        }

        Command::Delete { all, ordinal } => {
            let items = manager.get_items()?;

            if *all {
                for item in items.iter() {
                    manager.delete_item(item)?;
                }

                println!("Deleted all items.");
            } else {
                let item = items.get(ordinal.unwrap() - 1);
                manager.delete_item(item.unwrap())?;
                println!("Deleted \"{}\".", item.unwrap().title);
            }
        }

        Command::List => {
            let items = manager.get_items()?;

            if items.is_empty() {
                println!("No ToDos. All done!");
            } else {
                let mut ordinal = 1;

                items.iter().for_each(|item| {
                    let title = &item.title;
                    let due_date = &item
                        .due_date
                        .with_timezone(&Local)
                        .format("%H:%M, %d.%m.%Y");

                    println!("{}. {} (Due by {})", ordinal, title, due_date);
                    ordinal += 1;
                });
            }
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
