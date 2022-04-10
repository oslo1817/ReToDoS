use chrono::{DateTime, Utc};
use redis::{from_redis_value, FromRedisValue, RedisResult, Value};
use std::collections::hash_map::HashMap;

/// A ToDo item with a title and due date.
#[derive(Debug, Hash)]
pub struct ToDoItem {
    /// The title of the ToDo item.
    pub title: String,

    /// The due date of the ToDo item.
    pub due_date: DateTime<Utc>,
}

impl FromRedisValue for ToDoItem {
    fn from_redis_value(value: &Value) -> RedisResult<ToDoItem> {
        let map: HashMap<String, String> = from_redis_value(value)?;

        let title = map.get("title").unwrap().to_string();
        let due_date = DateTime::parse_from_rfc3339(map.get("due_date").unwrap())
            .unwrap()
            .with_timezone(&Utc);

        Ok(ToDoItem { title, due_date })
    }
}
