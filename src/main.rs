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
                let ordinal = ordinal.unwrap_or(0);
                let item = ordinal.checked_sub(1).and_then(|i| items.get(i));

                if let Some(item) = item {
                    manager.delete_item(item)?;
                    println!("Deleted \"{}\".", item.title);
                } else {
                    Err(format!("No item at ordinal {}.", ordinal))?;
                }
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
        let due_date = parse_date_string(due_date, Local::now(), Dialect::Uk)?.with_timezone(&Utc);

        Ok(Self { title, due_date })
    }
}
