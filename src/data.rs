pub mod model;

use model::ToDoItem;
use redis::{Client, Connection, RedisResult};
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
        self.add_item_hash(item)?;
        self.add_item_index(item)
    }

    /// Adds the supplied [item] as hash to the list of ToDo items.
    fn add_item_hash(&mut self, item: &ToDoItem) -> RedisResult<()> {
        redis::cmd("HSET")
            .arg(format!("retodos/items/{}", item.get_default_hash()))
            .arg(item)
            .query(self.connect()?)
    }

    /// Adds the supplied [item] to the index of ToDo items.
    fn add_item_index(&mut self, item: &ToDoItem) -> RedisResult<()> {
        redis::cmd("ZADD")
            .arg("retodos/items/index")
            .arg(item.due_date.timestamp())
            .arg(item.get_default_hash())
            .query(self.connect()?)
    }

    /// Query the sorted list of ToDo items.
    pub fn get_items(&mut self) -> RedisResult<Vec<ToDoItem>> {
        self.get_item_indices()?
            .iter()
            .map(|index| self.get_item(&format!("retodos/items/{}", index)))
            .collect()
    }

    /// Query the ToDo item at the specified [key].
    pub fn get_item(&mut self, key: &String) -> RedisResult<ToDoItem> {
        redis::cmd("HGETALL").arg(key).query(self.connect()?)
    }

    /// Queries the index (i.e. the hash value) of each ToDo item.
    pub fn get_item_indices(&mut self) -> RedisResult<Vec<String>> {
        redis::cmd("ZRANGE")
            .arg("retodos/items/index")
            .arg(0)
            .arg(-1)
            .query(self.connect()?)
    }

    /// Queries information from Redis using `INFO [section]`.
    pub fn get_redis_info(&mut self, section: &String) -> RedisResult<String> {
        Ok(redis::cmd("INFO").arg(section).query(self.connect()?)?)
    }
}

impl ToDoItem {
    /// Computes a hash value with [std::collections::hash_map::DefaultHasher].
    fn get_default_hash(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        let mut hasher = DefaultHasher::new();

        self.hash(&mut hasher);
        hasher.finish()
    }
}
