pub mod model;

use model::ToDoItem;
use redis::{Client, Cmd, Connection, RedisResult};
use std::hash::{Hash, Hasher};

/// A manager for a connection to a Redis server.
pub struct Manager {
    /// The current connection to the Redis server.
    connection: Option<Connection>,
}

impl Manager {
    /// Creates a new manager in a disconnected state.
    pub fn new() -> Manager {
        Manager { connection: None }
    }

    /// Connects to the Redis server, if not already connected, and returns the connection.
    pub fn connect(&mut self) -> RedisResult<&mut Connection> {
        if self.connection.is_none() {
            // Connect to the Redis server at `localhost:6379`.
            let client = Client::open("redis://127.0.0.1/")?;
            self.connection = Some(client.get_connection()?);
        }

        Ok(self.connection.as_mut().unwrap())
    }

    /// Adds the supplied [item] to the list of ToDo items.
    pub fn add_item(&mut self, item: &ToDoItem) -> RedisResult<()> {
        let mut command = redis::cmd("HSET");

        item.write_to(&mut command, "retodos/items");
        command.query(self.connect()?)?;

        Ok(())
    }

    /// Queries information from Redis using `INFO [section]`.
    pub fn get_redis_info(&mut self, section: &String) -> RedisResult<String> {
        Ok(redis::cmd("INFO").arg(section).query(self.connect()?)?)
    }
}

impl ToDoItem {
    /// Writes the item to the supplied [command] with the specified [prefix].
    fn write_to(&self, command: &mut Cmd, prefix: &str) {
        use std::collections::hash_map::DefaultHasher;
        let mut hasher = DefaultHasher::new();

        self.hash(&mut hasher);
        let key = format!("{}{}", prefix, hasher.finish());

        command.arg(key);
        command.arg("title").arg(&self.title);
        command.arg("due_data").arg(self.due_date.to_rfc3339());
    }
}
